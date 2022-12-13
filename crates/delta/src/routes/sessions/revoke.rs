use revolt_quark::{models::Session, Database, Error, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

#[openapi(skip)]
#[delete("/<id>")]
pub async fn req(db: &State<Database>, user: Session, id: String) -> Result<EmptyResponse> {
    let session = db.find_session(&id).await?;

    if session.user_id != user.user_id {
        return Err(Error::InvalidOperation);
    }

    db.delete_session(&id).await.map(|_| EmptyResponse)
}
