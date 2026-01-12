//! NAT Traversal and STUN support
//!
//! Implements basic RFC 5389 STUN Binding Requests to discover public IP/Port mapping.

use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::time::Duration;
use thiserror::Error;
use rand::Rng;
use bytes::{Buf, BufMut, BytesMut};

#[derive(Error, Debug)]
pub enum NatError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("STUN request timed out")]
    Timeout,
    #[error("Invalid STUN response")]
    InvalidResponse,
    #[error("Unknown address family")]
    UnknownAddressFamily,
}

pub type Result<T> = std::result::Result<T, NatError>;

const STUN_MAGIC_COOKIE: u32 = 0x2112A442;
const STUN_METHOD_BINDING: u16 = 0x0001;
const STUN_CLASS_REQUEST: u16 = 0x0000;
const STUN_CLASS_SUCCESS_RESPONSE: u16 = 0x0100;

const STUN_ATTR_MAPPED_ADDRESS: u16 = 0x0001;
const STUN_ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;

/// STUN Client for NAT discovery
pub struct StunClient {
    socket: UdpSocket,
}

impl StunClient {
    /// Create a new STUN client bound to a local port
    pub async fn bind(local_port: u16) -> Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", local_port)).await?;
        Ok(Self { socket })
    }

    /// Perform a STUN Binding Request to discover public address
    pub async fn discover_public_ip(&self, stun_server: &str) -> Result<SocketAddr> {
        let server_addr: SocketAddr = stun_server.parse()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid STUN server address"))?;

        // 1. Build Binding Request
        let tx_id = rand::random::<u128>();
        let packet = self.build_binding_request(tx_id);

        // 2. Send
        self.socket.send_to(&packet, server_addr).await?;

        // 3. Wait for response (retry logic omitted for MVP)
        let mut buf = [0u8; 1024];
        let (len, _src) = tokio::time::timeout(Duration::from_secs(3), self.socket.recv_from(&mut buf))
            .await
            .map_err(|_| NatError::Timeout)??;

        // 4. Parse Response
        self.parse_binding_response(&buf[0..len], tx_id)
    }

    fn build_binding_request(&self, tx_id: u128) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(20);
        
        // Message Type (Binding Request: 0x0001)
        buf.put_u16(STUN_METHOD_BINDING | STUN_CLASS_REQUEST);
        // Message Length (0 for no attributes)
        buf.put_u16(0);
        // Magic Cookie
        buf.put_u32(STUN_MAGIC_COOKIE);
        // Transaction ID (12 bytes)
        buf.put_u128(tx_id);

        buf.to_vec()
    }

    fn parse_binding_response(&self, data: &[u8], expected_tx_id: u128) -> Result<SocketAddr> {
        if data.len() < 20 {
            return Err(NatError::InvalidResponse);
        }

        let mut reader = &data[0..];
        let msg_type = reader.get_u16();
        let msg_len = reader.get_u16() as usize;
        let magic_cookie = reader.get_u32();
        let tx_id = reader.get_u128();

        if magic_cookie != STUN_MAGIC_COOKIE || tx_id != expected_tx_id {
            return Err(NatError::InvalidResponse);
        }

        // Expect Binding Success Response (0x0101)
        if msg_type != (STUN_METHOD_BINDING | STUN_CLASS_SUCCESS_RESPONSE) {
            return Err(NatError::InvalidResponse);
        }

        // Parse attributes
        let mut attr_reader = &data[20..20+msg_len];
        while attr_reader.remaining() >= 4 {
            let attr_type = attr_reader.get_u16();
            let attr_len = attr_reader.get_u16() as usize;
            
            // Padding handling
            let padding = (4 - (attr_len % 4)) % 4;
            
            if attr_reader.remaining() < attr_len {
                break;
            }

            match attr_type {
                STUN_ATTR_MAPPED_ADDRESS => {
                    let _reserved = attr_reader.get_u8();
                    let family = attr_reader.get_u8();
                    let port = attr_reader.get_u16();
                    
                    let ip = match family {
                        0x01 => { // IPv4
                            let ip_bytes = attr_reader.get_u32();
                            std::net::IpAddr::V4(std::net::Ipv4Addr::from(ip_bytes))
                        },
                        0x02 => { // IPv6
                            let mut ip_bytes = [0u8; 16];
                            attr_reader.copy_to_slice(&mut ip_bytes);
                            std::net::IpAddr::V6(std::net::Ipv6Addr::from(ip_bytes))
                        },
                        _ => return Err(NatError::UnknownAddressFamily),
                    };
                    
                    // Skip padding if any (though mapped address is aligned usually)
                    if padding > 0 && attr_reader.remaining() >= padding {
                        attr_reader.advance(padding);
                    }
                    
                    return Ok(SocketAddr::new(ip, port));
                },
                STUN_ATTR_XOR_MAPPED_ADDRESS => {
                    let _reserved = attr_reader.get_u8();
                    let family = attr_reader.get_u8();
                    let x_port = attr_reader.get_u16();
                    let port = x_port ^ ((STUN_MAGIC_COOKIE >> 16) as u16);
                    
                    let ip = match family {
                        0x01 => { // IPv4
                            let x_ip = attr_reader.get_u32();
                            let ip_u32 = x_ip ^ STUN_MAGIC_COOKIE;
                            std::net::IpAddr::V4(std::net::Ipv4Addr::from(ip_u32))
                        },
                        0x02 => { // IPv6
                            let mut x_ip = [0u8; 16];
                            attr_reader.copy_to_slice(&mut x_ip);
                            // XOR logic for IPv6 omitted for brevity in MVP
                            // Just return dummy or error
                            return Err(NatError::UnknownAddressFamily); 
                        },
                        _ => return Err(NatError::UnknownAddressFamily),
                    };
                    
                    return Ok(SocketAddr::new(ip, port));
                },
                _ => {
                    // Unknown attribute, skip
                    if attr_reader.remaining() >= attr_len {
                        attr_reader.advance(attr_len);
                    }
                }
            }
            
            // Handle padding for skipped attributes
            if padding > 0 && attr_reader.remaining() >= padding {
                attr_reader.advance(padding);
            }
        }

        Err(NatError::InvalidResponse)
    }
}
