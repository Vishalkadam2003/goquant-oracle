🔷 1. System Requirements

Minimum:

Rust (1.75+)

Cargo

Anchor (0.29+)

Solana CLI tools

PostgreSQL (v15+)

Redis (latest)

Git

Recommended OS:
Linux / macOS / Windows (WSL or native).

🔷 2. Repository Structure
goquant-oracle/
│
├── programs/oracle       # Solana Anchor program
├── backend/              # Rust oracle backend
├── migrations/           # PostgreSQL schema
├── docs/                 # Documentation
└── README.md

🔷 3. Environment Setup
3.1 Install Rust
curl https://sh.rustup.rs -sSf | sh


Verify:

rustc --version

3.2 Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"


Verify:

solana --version

3.3 Install Anchor Framework
cargo install --git https://github.com/coral-xyz/anchor anchor-cli


Verify:

anchor --version

3.4 Install PostgreSQL

Windows: Installed via official installer
Linux:

sudo apt install postgresql postgresql-contrib


Start service:

sudo service postgresql start

3.5 Install Redis

Windows (Docker):

docker run -p 6379:6379 redis


Linux:

sudo apt install redis-server


Test:

redis-cli ping

🔷 4. Configure Database
4.1 Create database
createdb oracle


Or from inside psql:

CREATE DATABASE oracle;

4.2 Run migrations
psql -U postgres -d oracle -f migrations/001_create_price_history.sql
psql -U postgres -d oracle -f migrations/002_create_oracle_health.sql
psql -U postgres -d oracle -f migrations/003_create_deviation_alerts.sql
psql -U postgres -d oracle -f migrations/004_create_source_reliability.sql

🔷 5. Backend Configuration

The backend reads config from environment variables or defaults.

5.1 Default Configuration

File: config.rs

Self {
    redis_url: "redis://127.0.0.1/".to_string(),
    postgres_url: "postgres://user:pass@localhost/oracle".to_string(),
    solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
    symbols: vec!["BTC".to_string(), "ETH".to_string()],
}

5.2 Customize Configuration

Create .env file:

REDIS_URL=redis://127.0.0.1/
POSTGRES_URL=postgres://postgres:yourpassword@localhost/oracle
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SYMBOLS=BTC,ETH,SOL


Use dotenv crate if needed.

🔷 6. Running the Backend

Move to backend folder:

cd backend
cargo run


Expected output:

Starting Oracle Backend Service...
OracleManager started...
HTTP server listening on 0.0.0.0:8080


The service now:

Fetches oracle prices (stubbed or real)

Aggregates consensus price

Writes price to Redis + Postgres

Serves API & WebSockets at port 8080

🔷 7. Testing the API
Consensus price:
curl http://localhost:8080/oracle/price/BTC

Historical data:
curl http://localhost:8080/oracle/history/BTC

Source-level values:
curl http://localhost:8080/oracle/sources/BTC

WebSocket stream:
ws://localhost:8080/ws/prices/BTC

🔷 8. Anchor Program Deployment (Optional for assignment)
Build program:
cd programs/oracle
anchor build

Localnet Test:
solana-test-validator
anchor deploy


This step is not required in the assignment, but the program is fully structured and ready.

🔷 9. Production Deployment (High-Level)

This can be mentioned in the documentation/video:

Deploy backend as systemd service or Docker container

Use Redis cluster for high availability

Use PostgreSQL + TimescaleDB for time-series optimization

Use Kubernetes for scale

Use Prometheus + Grafana for monitoring

Use NGINX for API gateway + SSL

These details help demonstrate real-world awareness.

🔷 10. Monitoring Tools

Use:

redis-cli monitor

pg_stat_activity

Solana Explorer for RPC health

Tracing logs via tracing_subscriber