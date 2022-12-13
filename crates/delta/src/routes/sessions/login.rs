use revolt_quark::{models::Session, Database, Error, Result};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use validator::Validate;

use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    address: String,

    username: String,

    iss: String,

    exp: usize,

    nbf: usize,
}

#[derive(Validate, Serialize, Deserialize, JsonSchema)]
pub struct DataLogin {
    #[validate(length(min = 8, max = 1024))]
    jwt: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "result")]
pub struct ResponseLogin {
    token: String,
}

#[openapi(skip)]
#[post("/login", data = "<data>")]
pub async fn req(db: &State<Database>, data: Json<DataLogin>) -> Result<Json<ResponseLogin>> {
    let data = data.into_inner();
    data.validate()
        .map_err(|error| Error::FailedValidation { error })?;

    let ret = decode::<Token>(
        &data.jwt,
        &DecodingKey::from_secret("rZ-A+nq!f".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidOperation)?;

    let session = Session {
        id: ulid::Ulid::new().to_string(),
        token: nanoid!(64),
        user_id: ret.claims.address,
        name: ret.claims.username,
        subscription: None,
    };

    db.save_session(&session).await?;

    Ok(Json(ResponseLogin {
        token: session.token,
    }))
}
