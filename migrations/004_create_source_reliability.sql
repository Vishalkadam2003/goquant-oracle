CREATE TABLE IF NOT EXISTS source_reliability (
    id BIGSERIAL PRIMARY KEY,
    source TEXT NOT NULL,          -- pyth or switchboard
    symbol TEXT,
    window_start TIMESTAMPTZ NOT NULL,
    window_end TIMESTAMPTZ NOT NULL,
    uptime_percent NUMERIC(5, 2),
    avg_latency_ms NUMERIC(10, 2),
    num_failures INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_source_reliability_source
    ON source_reliability(source);

CREATE INDEX IF NOT EXISTS idx_source_reliability_symbol
    ON source_reliability(symbol);

