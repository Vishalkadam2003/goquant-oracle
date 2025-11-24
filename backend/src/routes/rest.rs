use axum::{Router, routing::get, Json};
use axum::extract::{State, Path};
use serde_json::json;

use crate::SharedState;

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/prices", get(list_prices))
        .route("/price/:symbol", get(get_price))
        .route("/health", get(health))
        .route("/history/:symbol", get(get_history))
        .route("/sources/:symbol", get(get_sources))


        .with_state(state)
}

async fn list_prices(
    State(state): State<SharedState>,
) -> Json<serde_json::Value> {

    let mut out = serde_json::Map::new();

    for sym in ["BTC", "ETH"] {
        if let Ok(Some(price)) = state.cache.get_price(sym).await {
            out.insert(sym.to_string(), json!(price));
        }
    }

    Json(json!(out))
}

async fn get_price(
    Path(symbol): Path<String>,
    State(state): State<SharedState>,
) -> Json<serde_json::Value> {

    if let Ok(Some(price)) = state.cache.get_price(&symbol).await {
        return Json(json!(price));
    }

    Json(json!({ "error": "not_found" }))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }
async fn get_history(
    axum::extract::Path(symbol): axum::extract::Path<String>,
    state: axum::extract::State<SharedState>,
) -> Json<serde_json::Value> {
    match state.db.fetch_history(&symbol).await {
        Ok(rows) => Json(serde_json::json!(rows)),
        Err(_) => Json(serde_json::json!({ "error": "db_error" })),
    }
}
async fn get_sources(
    axum::extract::Path(symbol): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    // Mock until real Pyth/Switchboard implemented
    Json(serde_json::json!({
        "symbol": symbol,
        "sources": [
            { "source": "pyth", "price": 50000.0, "confidence": 10.0 },
            { "source": "switchboard", "price": 49950.0, "confidence": 15.0 }
        ]
    }))
}

))
}
