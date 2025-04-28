use chrono::Utc;
use pog_common::repos::channel_comment_repository::{
    ConversationComment, ConversationCommentRepository,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

const SECONDS_BETWEEN_DB_WRITE: u64 = 10;
pub async fn summarize<R: ConversationCommentRepository>(list: ConversationList, repo: R) {
    loop {
        sleep(Duration::from_secs(SECONDS_BETWEEN_DB_WRITE)).await;
        let conversations = {
            let mut hm = list.conversations.lock().await;
            let conversations = hm.clone();
            hm.clear();
            conversations
        };
        if let Err(err) = repo.add_comments(conversations).await {
            println!("Error adding comments: {:?}", err);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConversationList {
    conversations: Arc<Mutex<HashMap<String, Vec<ConversationComment>>>>,
}

impl ConversationList {
    pub async fn add_conversation(
        &self,
        comment_id: &str,
        user_id: &str,
        channel: &str,
        user: &str,
        comment: &str,
    ) {
        let comment_id = comment_id.parse().unwrap();
        let channel_id = channel.parse().unwrap();
        let user_id = user_id.parse().unwrap();
        let conversation_comment = ConversationComment {
            comment_id,
            user_id,
            comment: comment.to_string(),
            time: Utc::now(),
            user_name: user.to_string(),
            channel_id,
        };
        let mut unlocked_conversations = self.conversations.lock().await;
        match unlocked_conversations.get_mut(channel) {
            Some(conversation) => {
                conversation.push(conversation_comment);
            }
            None => {
                unlocked_conversations.insert(channel.to_string(), vec![conversation_comment]);
            }
        };
    }
}
//
// #[derive(Clone,Debug)]
// pub struct Conversation {
//     pub comments: Vec<ConversationComment>,
// }
//
// impl Conversation {
//     pub fn serialize(&self) -> String {
//         let comments: Vec<String> = self.comments.iter().map(|c| c.comment.clone()).collect();
//         serde_json::to_string(&comments).unwrap()
//     }
// }
