CREATE TABLE IF NOT EXISTS price_history (
    id BIGSERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    price NUMERIC(38, 12) NOT NULL,
    timestamp BIGINT NOT NULL,
    sources_used INT NOT NULL,
    inserted_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for fast queries per symbol
CREATE INDEX IF NOT EXISTS idx_price_history_symbol
    ON price_history(symbol);

-- Index for time-based lookups
CREATE INDEX IF NOT EXISTS idx_price_history_timestamp
    ON price_history(timestamp);
