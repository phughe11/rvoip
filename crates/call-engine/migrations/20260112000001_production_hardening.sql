-- Production Hardening Migration

-- 1. Enable WAL mode for better concurrency
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;

-- 2. Add skills support to agents table if it doesn't exist
-- SQLite doesn't support ALTER TABLE ... ADD COLUMN ... IF NOT EXISTS in all versions, 
-- but we can safely try to add it. Since this is a migration, it will run once.
ALTER TABLE agents ADD COLUMN skills TEXT;
ALTER TABLE agents ADD COLUMN last_active DATETIME;

-- 3. Create call_history table for real metrics (SL, AHT, etc.)
CREATE TABLE IF NOT EXISTS call_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    call_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    agent_id TEXT,
    queue_id TEXT,
    caller_id TEXT,
    start_time DATETIME NOT NULL,
    answer_time DATETIME,
    end_time DATETIME,
    wait_time_seconds INTEGER,
    talk_time_seconds INTEGER,
    disposition TEXT CHECK (disposition IN ('answered', 'abandoned', 'timeout', 'error', 'transfer')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_call_history_agent ON call_history(agent_id);
CREATE INDEX IF NOT EXISTS idx_call_history_queue ON call_history(queue_id);
CREATE INDEX IF NOT EXISTS idx_call_history_time ON call_history(start_time);
CREATE INDEX IF NOT EXISTS idx_call_history_call_id ON call_history(call_id);
