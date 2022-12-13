use revolt_quark::{models::Session, Database, Error, Result};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

use super::fetch_all::SessionInfo;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataEditSession {
    /// Session friendly name
    pub friendly_name: String,
}

#[openapi(skip)]
#[patch("/<id>", data = "<data>")]
pub async fn req(
    db: &State<Database>,
    user: Session,
    id: String,
    data: Json<DataEditSession>,
) -> Result<Json<SessionInfo>> {
    let mut session = db.find_session(&id).await?;

    // Make sure we own this session
    if user.user_id != session.user_id {
        return Err(Error::InvalidSession);
    }

    // Rename the session
    session.name = data.into_inner().friendly_name;

    // Save session
    db.save_session(&session).await?;

    Ok(Json(session.into()))
}
