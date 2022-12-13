use revolt_quark::{models::Session, Database, Result};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SessionInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
}

impl From<Session> for SessionInfo {
    fn from(item: Session) -> Self {
        SessionInfo {
            id: item.id,
            name: item.name,
        }
    }
}

#[openapi(skip)]
#[get("/all")]
pub async fn req(db: &State<Database>, session: Session) -> Result<Json<Vec<SessionInfo>>> {
    db.find_sessions(&session.user_id)
        .await
        .map(|ok| ok.into_iter().map(|session| session.into()).collect())
        .map(Json)
}
