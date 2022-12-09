use okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use crate::models::user::UserHint;
use crate::models::Session;
use crate::models::User;
use crate::{Database, Error};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;

    #[allow(clippy::collapsible_match)]
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session: &Option<Session> = request
            .local_cache_async(async {
                let db = request.rocket().state::<Database>().expect("`Database`");

                let header_session_token = request
                    .headers()
                    .get("x-session-token")
                    .next()
                    .map(|x| x.to_string());

                if let Some(token) = header_session_token {
                    if let Ok(session) = db.find_session_by_token(&token).await {
                        return Some(session);
                    }
                }

                None
            })
            .await;

        if let Some(session) = session {
            Outcome::Success(session.clone())
        } else {
            Outcome::Failure((Status::Unauthorized, Error::InvalidSession))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user: &Option<User> = request
            .local_cache_async(async {
                let db = request.rocket().state::<Database>().expect("`Database`");

                let header_bot_token = request
                    .headers()
                    .get("x-bot-token")
                    .next()
                    .map(|x| x.to_string());

                if let Some(bot_token) = header_bot_token {
                    if let Ok(user) = User::from_token(db, &bot_token, UserHint::Bot).await {
                        return Some(user);
                    }
                } else if let Outcome::Success(session) = request.guard::<Session>().await {
                    // This uses a guard so can't really easily be refactored into from_token at this stage.
                    if let Ok(user) = db.fetch_user(&session.user_id).await {
                        return Some(user);
                    }
                }

                None
            })
            .await;

        if let Some(user) = user {
            Outcome::Success(user.clone())
        } else {
            Outcome::Failure((Status::Unauthorized, Error::InvalidSession))
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for User {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut requirements = schemars::Map::new();
        requirements.insert("Session Token".to_owned(), vec![]);

        Ok(RequestHeaderInput::Security(
            "Session Token".to_owned(),
            SecurityScheme {
                data: SecuritySchemeData::ApiKey {
                    name: "x-session-token".to_owned(),
                    location: "header".to_owned(),
                },
                description: Some("Used to authenticate as a user.".to_owned()),
                extensions: schemars::Map::new(),
            },
            requirements,
        ))
    }
}
