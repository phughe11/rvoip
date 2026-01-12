//! SIP Proxy Core Component
//!
//! # Status
//! 🚧 **Under Construction**
//!
//! This crate implements SIP proxy functionality including:
//! - Stateless packet routing
//! - Transaction stateful routing
//! - Load balancing logic

use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{info, debug, warn, error};
use anyhow::Result;

use rvoip_dialog_core::transaction::{TransactionManager, TransactionEvent};
use rvoip_sip_core::{Request, Response, Method, StatusCode, ResponseBuilder};

mod load_balancer;
use load_balancer::{LoadBalancer, RoundRobinLoadBalancer};

/// Proxy Server
pub struct ProxyServer {
    transaction_manager: Arc<TransactionManager>,
    running: bool,
    location_service: Arc<dyn LocationService>,
    dns_resolver: Arc<dyn DnsResolver>,
    load_balancer: Arc<dyn LoadBalancer>,
}

impl ProxyServer {
    /// Create a new Proxy Server
    pub fn new(transaction_manager: Arc<TransactionManager>) -> Self {
        Self {
            transaction_manager,
            running: false,
            location_service: Arc::new(InMemoryLocationService::new()),
            dns_resolver: Arc::new(BasicDnsResolver::new()),
            load_balancer: Arc::new(RoundRobinLoadBalancer::new()),
        }
    }
    
    // ... [start and handle_event methods remain unchanged]

    /// Start the proxy server loop
    pub async fn start(&mut self, mut event_rx: mpsc::Receiver<TransactionEvent>) -> Result<()> {
        info!("Starting SIP Proxy Server...");
        self.running = true;

        while self.running {
            tokio::select! {
                event = event_rx.recv() => {
                    match event {
                        Some(e) => {
                            if let Err(err) = self.handle_event(e).await {
                                error!("Error handling proxy event: {}", err);
                            }
                        }
                        None => {
                            info!("Proxy event channel closed. Stopping.");
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle incoming transaction events
    async fn handle_event(&self, event: TransactionEvent) -> Result<()> {
        match event {
            TransactionEvent::InviteRequest { request, source, transaction_id } => {
                info!("Proxy received INVITE from {}", source);
                self.forward_request(request, transaction_id).await?;
            }
            TransactionEvent::NonInviteRequest { request, source, transaction_id } => {
                info!("Proxy received {} from {}", request.method(), source);
                
                if request.method() == Method::Register {
                    self.handle_register(request, source, transaction_id).await?;
                } else {
                    self.forward_request(request, transaction_id).await?;
                }
            }
            // Add other event handlers as needed
            _ => {
                debug!("Proxy ignored event: {:?}", event);
            }
        }
        Ok(())
    }
    
    /// Forward a request statefully
    async fn forward_request(&self, request: Request, _server_tx_id: rvoip_dialog_core::transaction::TransactionKey) -> Result<()> {
        let uri = request.uri();
        info!("Forwarding request to: {}", uri);
        
        // 1. Resolve Destination
        // Order: Location Service (if User) -> DNS (if Domain) -> IP Direct
        let destination = self.resolve_destination(&uri).await?;
        
        info!("Resolved destination: {}", destination);

        // 2. Create Client Transaction
        let tx_id = self.transaction_manager.create_client_transaction(request, destination).await?;
        
        // 3. Send the request
        self.transaction_manager.send_request(&tx_id).await?;
        
        info!("Forwarded request via transaction: {}", tx_id);
        
        Ok(())
    }
    
    /// Handle REGISTER requests
    async fn handle_register(
        &self, 
        request: Request, 
        source: SocketAddr, 
        tx_id: rvoip_dialog_core::transaction::TransactionKey
    ) -> Result<()> {
        let user = request.to()
            .and_then(|t| t.address().uri.username())
            .map(|s| s.to_string())
            .unwrap_or_default();

        if !user.is_empty() {
             info!("LocationService: Registering {} -> {}", user, source);
             self.location_service.register(&user, source).await;
        }

        // Build 200 OK Response
        // We manually copy headers to ensure it matches the request transaction
        let mut builder = ResponseBuilder::new(StatusCode::Ok, None);
        
        if let Some(h) = request.from() { builder = builder.header(rvoip_sip_core::TypedHeader::From(h.clone())); }
        if let Some(h) = request.to() { builder = builder.header(rvoip_sip_core::TypedHeader::To(h.clone())); }
        if let Some(h) = request.call_id() { builder = builder.header(rvoip_sip_core::TypedHeader::CallId(h.clone())); }
        if let Some(h) = request.cseq() { builder = builder.header(rvoip_sip_core::TypedHeader::CSeq(h.clone())); }
        
        for via in request.via_headers() {
             builder = builder.header(rvoip_sip_core::TypedHeader::Via(via));
        }

        let response = builder.build();
        
        self.transaction_manager.send_response(&tx_id, response).await?;
        Ok(())
    }
    
    /// Resolve destination from URI
    async fn resolve_destination(&self, uri: &rvoip_sip_core::Uri) -> Result<SocketAddr> {
        // 1. Try Location Service (User Lookup)
        // If the URI has a user part, check if it's a registered user
        if let Some(user) = uri.username() {
             if let Some(contact) = self.location_service.lookup(user).await {
                 info!("Location Service hit for user {}: {}", user, contact);
                 return Ok(contact);
             }
        }
        
        // 2. Try IP Parsing
        let host = &uri.host;
        let port = uri.port.unwrap_or(5060);
        
        if let Ok(ip) = host.to_string().parse::<std::net::IpAddr>() {
            return Ok(SocketAddr::new(ip, port));
        }
        
        // 3. Try DNS Resolution
        info!("Resolving domain via DNS: {}", host);
        match self.dns_resolver.resolve(&host.to_string(), port).await {
            Ok(addr) => {
                // Apply load balancing (Architectural Placeholder)
                // In a real scenario, DNS SRV would return multiple targets
                let candidates = vec![addr]; 
                if let Some(selected) = self.load_balancer.select_destination(&candidates).await {
                    return Ok(selected);
                }
                Ok(addr)
            },
            Err(e) => {
                anyhow::bail!("Failed to resolve target {}: {}", host, e);
            }
        }
    }
}

/// Location Service Trait
#[async_trait::async_trait]
pub trait LocationService: Send + Sync {
    async fn lookup(&self, user: &str) -> Option<SocketAddr>;
    async fn register(&self, user: &str, contact: SocketAddr);
}

use dashmap::DashMap;

/// Basic In-Memory Location Service
pub struct InMemoryLocationService {
    // Map of User -> Contact Address
    registry: Arc<DashMap<String, SocketAddr>>,
}

impl InMemoryLocationService {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl LocationService for InMemoryLocationService {
    async fn lookup(&self, user: &str) -> Option<SocketAddr> {
        // Look up user in the registry
        if let Some(contact) = self.registry.get(user) {
            debug!("LocationService: Found contact for {}: {}", user, contact.value());
            return Some(*contact.value());
        }
        debug!("LocationService: No contact found for {}", user);
        None 
    }
    
    async fn register(&self, user: &str, contact: SocketAddr) {
        info!("LocationService: Registered {} at {}", user, contact);
        self.registry.insert(user.to_string(), contact);
    }
}

/// DNS Resolver Trait
#[async_trait::async_trait]
pub trait DnsResolver: Send + Sync {
    async fn resolve(&self, domain: &str, port: u16) -> Result<SocketAddr>;
}

/// Basic DNS Resolver (Stub)
pub struct BasicDnsResolver;

impl BasicDnsResolver {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl DnsResolver for BasicDnsResolver {
    async fn resolve(&self, domain: &str, port: u16) -> Result<SocketAddr> {
        // TODO: Implement SRV lookup: _sip._udp.domain
        
        // Fallback to basic A-record resolution (blocking for MVP convenience, strictly should be async)
        // In this stub, we just handle localhost
        if domain == "localhost" {
             return Ok(SocketAddr::new("127.0.0.1".parse().unwrap(), port));
        }
        
        // Or try system resolver
        use std::net::ToSocketAddrs;
        let addr_str = format!("{}:{}", domain, port);
        // Warning: This is blocking IO in async context, but acceptable for MVP stub
        if let Ok(mut iter) = addr_str.to_socket_addrs() {
            if let Some(addr) = iter.next() {
                return Ok(addr);
            }
        }
        
        anyhow::bail!("Domain resolution failed for {}", domain);
    }
}
