use revolt_quark::{
    models::{Session, WebPushSubscription},
    Database, EmptyResponse, Error, Result,
};

use rocket::State;

/// # Unsubscribe
///
/// Remove the Web Push subscription associated with the current session.
#[openapi(skip)]
#[post("/unsubscribe")]
pub async fn req(db: &State<Database>, mut session: Session) -> Result<EmptyResponse> {
    session.subscription = None;
    db.save_session(&session)
        .await
        .map(|_| EmptyResponse)
        .map_err(|_| Error::DatabaseError {
            operation: "save",
            with: "session",
        })
}
