CREATE TABLE IF NOT EXISTS oracle_health (
    id BIGSERIAL PRIMARY KEY,
    source TEXT NOT NULL,        -- pyth, switchboard, internal
    symbol TEXT,                 -- optional; some health is global
    status TEXT NOT NULL,        -- healthy, stale, failed
    last_update BIGINT NOT NULL, -- unix timestamp
    latency_ms INT,
    error_msg TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_oracle_health_source
    ON oracle_health(source);

CREATE INDEX IF NOT EXISTS idx_oracle_health_symbol
    ON oracle_health(symbol);
