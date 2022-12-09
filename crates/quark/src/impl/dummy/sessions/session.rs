use crate::models::Session;
use crate::{AbstractSession, Result};

use super::super::DummyDb;

#[async_trait]
impl AbstractSession for DummyDb {
    async fn find_session(&self, id: &str) -> Result<Session> {
        todo!("{id}")
    }

    async fn find_sessions(&self, user_id: &str) -> Result<Vec<Session>> {
        todo!("{user_id}")
    }

    async fn find_sessions_with_subscription(&self, user_ids: &[String]) -> Result<Vec<Session>> {
        todo!("{user_ids:?}")
    }

    async fn find_session_by_token(&self, token: &str) -> Result<Session> {
        todo!("{token}")
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        todo!("{session:?}")
    }

    async fn delete_session(&self, id: &str) -> Result<()> {
        todo!("{id}")
    }

    async fn delete_all_sessions(&self, user_id: &str, ignore: Option<String>) -> Result<()> {
        todo!("{user_id} {ignore:?}")
    }
}
