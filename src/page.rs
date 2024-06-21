use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageObject {
    pub component: String,
    pub props: serde_json::Value,
    pub url: String,
    pub version: String,
}
