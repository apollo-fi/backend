use rocket::State;
use rocket_empty::EmptyResponse;

#[openapi(tag = "Session")]
#[post("/logout")]
pub async fn logout(db: &State<Database>, session: Session) -> Result<EmptyResponse> {
    db.delete_session(&session.id).await.map(|_| EmptyResponse)
}
