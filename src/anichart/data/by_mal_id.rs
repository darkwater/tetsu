use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByMalIdParams {
    pub mal_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: i32,
}
