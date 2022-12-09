use revolt_quark::{
    models::{Session, WebPushSubscription},
    EmptyResponse, Error, Result,
};

use rocket::State;

/// # Unsubscribe
///
/// Remove the Web Push subscription associated with the current session.
#[openapi(skip)]
#[post("/unsubscribe")]
pub async fn req(/*rauth: &State<RAuth>, */ mut session: Session) -> Result<EmptyResponse> {
    /* TODO
    session.subscription = None;
    session
        .save(&rauth)
        .await
        .map(|_| EmptyResponse)
        .map_err(|_| Error::DatabaseError {
            operation: "save",
            with: "session",
        })
        */
    Ok(EmptyResponse)
}
