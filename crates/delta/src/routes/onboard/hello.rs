use revolt_quark::models::{Session, User};
use rocket::serde::json::Json;
use serde::Serialize;

/// # Onboarding Status
#[derive(Serialize, JsonSchema)]
pub struct DataHello {
    /// Whether onboarding is required
    onboarding: bool,
}

/// # Check Onboarding Status
///
/// This will tell you whether the current account requires onboarding or whether you can continue to send requests as usual. You may skip calling this if you're restoring an existing session.
#[openapi(skip)]
#[get("/hello")]
pub async fn req(_session: Session, user: Option<User>) -> Json<DataHello> {
    Json(DataHello {
        onboarding: user.is_none(),
    })
}
