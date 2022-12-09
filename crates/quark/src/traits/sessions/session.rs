use crate::models::Session;
use crate::Result;

#[async_trait]
pub trait AbstractSession: Sync + Send {
    async fn find_session(&self, id: &str) -> Result<Session>;

    async fn find_sessions(&self, user_id: &str) -> Result<Vec<Session>>;

    async fn find_sessions_with_subscription(&self, user_ids: &[String]) -> Result<Vec<Session>>;

    async fn find_session_by_token(&self, token: &str) -> Result<Session>;

    async fn save_session(&self, session: &Session) -> Result<()>;

    async fn delete_session(&self, id: &str) -> Result<()>;

    async fn delete_all_sessions(&self, user_id: &str, ignore: Option<String>) -> Result<()>;
}
