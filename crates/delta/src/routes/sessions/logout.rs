use revolt_quark::{models::Session, Database, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

#[openapi(skip)]
#[post("/logout")]
pub async fn req(db: &State<Database>, session: Session) -> Result<EmptyResponse> {
    db.delete_session(&session.id).await.map(|_| EmptyResponse)
}
