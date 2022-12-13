use revolt_quark::{
    models::{Session, WebPushSubscription},
    Database, EmptyResponse, Error, Result,
};

use rocket::{serde::json::Json, State};

/// # Push Subscribe
///
/// Create a new Web Push subscription.
///
/// If an existing subscription exists on this session, it will be removed.
#[openapi(skip)]
#[post("/subscribe", data = "<data>")]
pub async fn req(
    db: &State<Database>,
    mut session: Session,
    data: Json<WebPushSubscription>,
) -> Result<EmptyResponse> {
    session.subscription = Some(data.into_inner());
    db.save_session(&session)
        .await
        .map(|_| EmptyResponse)
        .map_err(|_| Error::DatabaseError {
            operation: "save",
            with: "session",
        })
}
