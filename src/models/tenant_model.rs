use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tenant {
    pub id: u64,
    pub tenant: String,
    pub authkey: String,
}