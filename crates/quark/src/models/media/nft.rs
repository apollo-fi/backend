use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
pub struct NFT {
    #[serde(rename = "_id")]
    pub id: String,

    pub address: String,

    pub token_id: String,

    pub image: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}
