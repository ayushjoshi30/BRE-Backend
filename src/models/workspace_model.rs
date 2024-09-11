use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct WorkspaceResponse {
    pub id: i32,
    pub name: String,
}
