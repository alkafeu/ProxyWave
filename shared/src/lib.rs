//! Shared types and utilities across backend and frontend.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppVersion {
    pub version: String,
} 