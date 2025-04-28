use crate::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ConversationComment {
    pub comment_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub channel_id: i64,
    pub comment: String,
}

pub trait ConversationCommentRepository {
    fn add_comments(
        &self,
        comments: HashMap<String, Vec<ConversationComment>>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn get_latest_from_channel(
        &self,
        channel_id: i64,
    ) -> impl std::future::Future<Output = Result<Vec<ConversationComment>, Error>> + Send;
}
