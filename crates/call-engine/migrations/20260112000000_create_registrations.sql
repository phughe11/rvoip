-- Create registrations table for persistent SIP registrations
CREATE TABLE IF NOT EXISTS registrations (
    aor TEXT PRIMARY KEY,
    contact_uri TEXT NOT NULL,
    expires_at DATETIME NOT NULL,
    user_agent TEXT,
    transport TEXT NOT NULL,
    remote_addr TEXT NOT NULL,
    last_updated DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Index for expiration cleanup
CREATE INDEX IF NOT EXISTS idx_registrations_expires_at ON registrations(expires_at);

-- Index for contact URI lookup (reverse lookup)
CREATE INDEX IF NOT EXISTS idx_registrations_contact ON registrations(contact_uri);
