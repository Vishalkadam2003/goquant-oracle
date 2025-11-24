 GoQuant Oracle Integration & Price Feed System
High-Performance Oracle Aggregation for Perpetual Futures DEX

Developer: Vishal Kadam
Tech Stack: Rust, PostgreSQL, Redis, WebSockets

 Overview

This project implements a secure, manipulation-resistant, and high-availability Oracle Integration System for a perpetual futures decentralized exchange (DEX).
The system integrates Pyth and Switchboard oracles, validates price consensus, aggregates price feeds, caches real-time updates, and serves them via REST and WebSocket APIs.

This forms the single source of truth for mark price, funding rate calculations, and liquidation triggers.

 Features

Read price feeds from Pyth and Switchboard

Validate:

Staleness (< 30s)

Confidence intervals

Price deviation limits

Compute median-based consensus price

Reject manipulated or outlier data

Configurable oracle rules per symbol

✅ Rust Backend Oracle Service

WebSocket subscription to live oracle feeds

Pyth & Switchboard client implementation

Median & confidence-weighted price aggregation

Redis-based real-time caching

PostgreSQL historical data storage

Failover logic + health monitoring

REST + WebSocket API layer

✅ High Reliability Engineering

Automatic reconnection to Solana RPC

Circuit breakers for unhealthy sources

Stale price detection system

99.9% uptime readiness via caching + fallback

🛠️ Installation & Setup
1. Install Dependencies
Prerequisites

Rust (1.75+)

Solana CLI Tools

Anchor Framework (0.29+)

PostgreSQL

Redis

Git

2. Clone Repository
git clone https://github.com/Vishalkadam2003/goquant-oracle
cd goquant-oracle

📦 Backend Setup
Install Rust Dependencies
cd oracle-services
cargo build

Start Backend Server
cargo run

⚙️ Environment Variables

Create .env in oracle-services/:

SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
PYTH_PRICE_FEED=...
SWITCHBOARD_AGGREGATOR=...
POSTGRES_URL=postgres://user:password@localhost:5432/oracle
REDIS_URL=redis://127.0.0.1:6379

🏗️ Database Setup
Run PostgreSQL migrations
psql -U <username> -d oracle -f migrations/init.sql


Tables include:

price_history

oracle_health

confidence_intervals

deviation_alerts

🔧 Running the Smart Contract (Anchor)
Build and Deploy Program
anchor build
anchor deploy

🌐 API Endpoints
Method	Endpoint	Description
GET	/oracle/price/:symbol	Current validated consensus price
GET	/oracle/prices	Fetch prices for all symbols
GET	/oracle/history/:symbol	Historical prices from PostgreSQL
GET	/oracle/sources/:symbol	Raw prices from Pyth + Switchboard
GET	/oracle/health	Oracle health and uptime
📡 WebSocket Streams

Real-time price updates

Oracle health alerts

Price confidence changes

High deviation warnings
