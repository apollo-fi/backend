use revolt_quark::{models::Session, Database, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

#[openapi(skip)]
#[delete("/all?<revoke_self>")]
pub async fn req(
    db: &State<Database>,
    session: Session,
    revoke_self: Option<bool>,
) -> Result<EmptyResponse> {
    let ignore = if revoke_self.unwrap_or(false) {
        None
    } else {
        Some(session.id)
    };

    db.delete_all_sessions(&session.user_id, ignore)
        .await
        .map(|_| EmptyResponse)
}
