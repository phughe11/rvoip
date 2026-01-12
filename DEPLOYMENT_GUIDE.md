# RVOIP Production Deployment Guide

This guide details how to deploy RVOIP (Rust VoIP Server) in a production environment.

## 📋 Prerequisites

- **OS**: Linux (Ubuntu 22.04 LTS / Debian 12 recommended) or macOS
- **CPU**: 2+ Cores (AES-NI support recommended for SRTP encryption)
- **RAM**: 4GB+
- **Network**: Public static IP address recommended for VoIP/RTP relay
- **Dependencies**:
  - `openssl` (libssl-dev)
  - `sqlite3` (libsqlite3-dev) or `libpq` (libpq-dev)
  - `pkg-config`

---

## 🛠️ 1. Building for Production

Do not use debug builds in production. Release builds are significantly faster and optimized.

```bash
# 1. Update dependencies
cargo update

# 2. Build release binary
# -p rvoip selects the main server binary
cargo build --release -p rvoip

# 3. Locate the binary
ls -lh target/release/rvoip
```

---

## 💾 2. Database Setup

RVOIP uses `sqlx` and supports SQLite (default) or PostgreSQL.

### Option A: SQLite (Simple, Embedded)

1.  **Install SQLx CLI**:
    ```bash
    cargo install sqlx-cli --no-default-features --features native-tls,sqlite
    ```

2.  **Create Database**:
    ```bash
    mkdir -p data
    export DATABASE_URL="sqlite:data/rvoip.db"
    sqlx database create
    ```

3.  **Run Migrations**:
    ```bash
    sqlx migrate run
    ```

### Option B: PostgreSQL (High Performance)

1.  **Environment Variable**:
    ```bash
    export DATABASE_URL="postgres://user:password@localhost/rvoip"
    ```

2.  **Run Migrations**:
    ```bash
    sqlx migrate run
    ```

---

## 🔒 3. TLS Configuration (SIPS & WSS)

Secure SIP (SIPS) and WebRTC (WSS) require valid TLS certificates.

### Directory Structure
```text
/etc/rvoip/
├── certs/
│   ├── server.crt  (Certificate Chain)
│   └── server.key  (Private Key)
└── config.toml
```

### Self-Signed (Testing Only)
```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

### Let's Encrypt (Production)
Use `certbot` to generate certificates, then point RVOIP config to `/etc/letsencrypt/live/yourdomain/`.

---

## ⚙️ 4. Configuration

Create `config.toml` in your working directory or `/etc/rvoip/`.

```toml
[server]
ip = "0.0.0.0"
sip_port = 5060
sips_port = 5061
ws_port = 8080
wss_port = 8443
domain = "sip.yourdomain.com"

[media]
# Public IP is CRITICAL for RTP relay
public_ip = "203.0.113.10" 
rtp_port_start = 10000
rtp_port_end = 20000

[database]
url = "sqlite:data/rvoip.db"

[tls]
cert_file = "certs/server.crt"
key_file = "certs/server.key"
```

---

## 🚀 5. Systemd Service Deployment

Create `/etc/systemd/system/rvoip.service`:

```ini
[Unit]
Description=RVOIP SIP Server
After=network.target postgresql.service

[Service]
Type=simple
User=rvoip
Group=rvoip
WorkingDirectory=/opt/rvoip
ExecStart=/opt/rvoip/rvoip
Restart=always
RestartSec=5
# Increase open file limit for high concurrency
LimitNOFILE=65535
# Environment variables
Environment=RUST_LOG=info
Environment=DATABASE_URL=sqlite:data/rvoip.db

[Install]
WantedBy=multi-user.target
```

**Enable and Start:**
```bash
useradd -r -s /bin/false rvoip
mkdir -p /opt/rvoip/data
chown -R rvoip:rvoip /opt/rvoip
# Copy binary to /opt/rvoip/
systemctl enable rvoip
systemctl start rvoip
```

---

## 🐳 6. Docker Deployment

### Dockerfile

See the provided `Dockerfile` in the root directory.

### Docker Compose

```yaml
version: '3.8'
services:
  rvoip:
    build: .
    network_mode: "host" # Recommended for VoIP to avoid NAT double-translation issues
    volumes:
      - ./data:/app/data
      - ./certs:/app/certs
      - ./config.toml:/app/config.toml
    environment:
      - RUST_LOG=info
      - DATABASE_URL=sqlite:data/rvoip.db
    restart: unless-stopped
```

**Run:**
```bash
docker-compose up -d --build
```

---

## 📊 7. Monitoring & Tuning

### Logs
RVOIP uses `tracing`. Set log levels via `RUST_LOG`:
- `error`: Critical errors only
- `info`: General operational info (Recommended)
- `debug`: Detailed SIP transaction flows
- `trace`: Full packet dumps (Do not use in prod)

### Kernel Tuning (sysctl.conf)
For high concurrent connections (10k+):

```bash
# Increase port range
net.ipv4.ip_local_port_range = 1024 65535
# Allow reusing sockets in TIME_WAIT
net.ipv4.tcp_tw_reuse = 1
# Increase max open files
fs.file-max = 100000
```

---

## ⚠️ Troubleshooting Common Issues

1.  **One-way Audio**:
    *   **Cause**: NAT/Firewall blocking RTP ports.
    *   **Fix**: Ensure `rtp_port_start` to `rtp_port_end` (UDP) are open in AWS/AWS SG/Firewall. Ensure `public_ip` is set correctly in `config.toml`.

2.  **Registration Timeout**:
    *   **Cause**: SIP port blocked or ALG interfering.
    *   **Fix**: Disable SIP ALG on router. Check port 5060 (UDP/TCP).

3.  **Database Locked (SQLite)**:
    *   **Cause**: High concurrency writes on slow disk.
    *   **Fix**: Use WAL mode (`PRAGMA journal_mode=WAL;`) or switch to PostgreSQL.
