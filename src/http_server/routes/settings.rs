use std::collections::HashMap;

use axum::{Form, Json};
use serde::Deserialize;

use crate::http_server::Result;

#[derive(Deserialize)]
pub struct SetRequest {
    key: String,
    value: serde_json::Value,
}

pub async fn get() -> Result<Json<HashMap<String, serde_json::Value>>> {
    Ok(Json(
        sqlx::query!("SELECT * FROM settings")
            .fetch_all(crate::DB.get().await)
            .await?
            .into_iter()
            .map(|r| (r.key, serde_json::from_str(&r.value).unwrap()))
            .collect(),
    ))
}

pub async fn post(Form(SetRequest { key, value }): Form<SetRequest>) -> Result<()> {
    let value = serde_json::to_string(&value)?;

    sqlx::query!(
        "INSERT INTO settings (key, value)
        VALUES ($1, $2)
        ON CONFLICT (key) DO UPDATE SET value = $2",
        key,
        value,
    )
    .execute(crate::DB.get().await)
    .await
    .map(|_| ())
    .map_err(|e| e.into())
}
