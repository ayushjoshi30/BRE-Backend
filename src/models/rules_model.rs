use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct RuleResponse {
    pub id: i32,
    pub rulejson: String,
}