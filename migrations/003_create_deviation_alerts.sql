CREATE TABLE IF NOT EXISTS deviation_alerts (
    id BIGSERIAL PRIMARY KEY,
    symbol TEXT NOT NULL,
    detected_at BIGINT NOT NULL,
    price NUMERIC(38, 12),
    deviation_bps INT NOT NULL,
    message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_deviation_alerts_symbol
    ON deviation_alerts(symbol);
