use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::anidb::records::Anime;

#[tarpc::service]
pub trait TetsuServer {
    async fn anime() -> Result<Vec<Anime>, Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    message: String,
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self { message: value.to_string() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}
