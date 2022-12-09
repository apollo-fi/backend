use bson::to_document;
use futures::TryStreamExt;
use mongodb::options::UpdateOptions;

use crate::models::Session;
use crate::{AbstractSession, Error, Result};

use super::super::MongoDb;

static COL: &str = "sessions";

#[async_trait]
impl AbstractSession for MongoDb {
    async fn find_session(&self, id: &str) -> Result<Session> {
        self.find_one_by_id(COL, id).await
    }

    async fn find_sessions(&self, user_id: &str) -> Result<Vec<Session>> {
        self.col::<Session>(COL)
            .find(
                doc! {
                    "user_id": user_id
                },
                None,
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "find",
                with: "sessions",
            })?
            .try_collect()
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "collect",
                with: "sessions",
            })
    }

    async fn find_sessions_with_subscription(&self, user_ids: &[String]) -> Result<Vec<Session>> {
        self.col::<Session>(COL)
            .find(
                doc! {
                    "user_id": {
                        "$in": user_ids
                    },
                    "subscription": {
                        "$exists": true
                    }
                },
                None,
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "find",
                with: "sessions",
            })?
            .try_collect()
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "collect",
                with: "sessions",
            })
    }

    async fn find_session_by_token(&self, token: &str) -> Result<Session> {
        self.col::<Session>(COL)
            .find_one(
                doc! {
                    "token": token
                },
                None,
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "find_one",
                with: "session",
            })?
            .ok_or(Error::UnknownUser)
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        self.col::<Session>(COL)
            .update_one(
                doc! {
                    "_id": &session.id
                },
                doc! {
                    "$set": to_document(session).map_err(|_| Error::DatabaseError {
                        operation: "to_document",
                        with: "session",
                    })?,
                },
                UpdateOptions::builder().upsert(true).build(),
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "upsert_one",
                with: "session",
            })
            .map(|_| ())
    }

    async fn delete_session(&self, id: &str) -> Result<()> {
        self.col::<Session>(COL)
            .delete_one(
                doc! {
                    "_id": id
                },
                None,
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "delete_one",
                with: "session",
            })
            .map(|_| ())
    }

    async fn delete_all_sessions(&self, user_id: &str, ignore: Option<String>) -> Result<()> {
        let mut query = doc! {
            "user_id": user_id
        };

        if let Some(id) = ignore {
            query.insert(
                "_id",
                doc! {
                    "$ne": id
                },
            );
        }

        self.col::<Session>(COL)
            .delete_many(query, None)
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "delete_one",
                with: "session",
            })
            .map(|_| ())
    }
}
