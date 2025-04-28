use crate::error::Error;
use crate::repos::channel_comment_repository::{
    ConversationComment, ConversationCommentRepository,
};
use chrono::NaiveDateTime;
use sqlx::{PgPool, Row};
use std::collections::HashMap;

const ADD_COMMENT: &str = r###"INSERT INTO
channel_comments(comment_id, user_id, user_name, time, channel_id, comment)
VALUES ($1, $2, $3, $4, $5, $6)"###;
const GET_LATEST_COMMENTS: &str = r###"SELECT
  comment_id,
  user_id,
  user_name,
  time,
  channel_id,
  comment
FROM channel_comments
WHERE channel_id = $1
ORDER BY time DESC
LIMIT 10"###;

#[derive(Clone, Debug)]
pub struct PostgresConversationCommentRepository {
    pool: PgPool,
}

impl PostgresConversationCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ConversationCommentRepository for PostgresConversationCommentRepository {
    async fn add_comments(
        &self,
        comments: HashMap<String, Vec<ConversationComment>>,
    ) -> Result<(), Error> {
        for channel in comments.values() {
            for comment in channel {
                sqlx::query(ADD_COMMENT)
                    .bind(comment.comment_id)
                    .bind(comment.user_id)
                    .bind(&comment.user_name)
                    .bind(comment.time)
                    .bind(comment.channel_id)
                    .bind(&comment.comment)
                    .execute(&self.pool)
                    .await?;
            }
        }
        Ok(())
    }

    async fn get_latest_from_channel(
        &self,
        channel_id: i64,
    ) -> Result<Vec<ConversationComment>, Error> {
        let rows = sqlx::query(GET_LATEST_COMMENTS)
            .bind(channel_id)
            .fetch_all(&self.pool)
            .await?;

        let comments = rows
            .iter()
            .map(|row| {
                let ntime: NaiveDateTime = row.get("time");
                ConversationComment {
                    comment_id: row.get("comment_id"),
                    user_id: row.get("user_id"),
                    user_name: row.get("user_name"),
                    time: ntime.and_utc(),
                    channel_id: row.get("channel_id"),
                    comment: row.get("comment"),
                }
            })
            .collect();

        Ok(comments)
    }
}
#[cfg(test)]
#[cfg(feature = "integration-tests")]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn repo_base_test() {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect("postgresql://pog_user:pog_pass@localhost:5432/pog_server")
            .await
            .expect("unable to connect to database");
        let repo = PostgresConversationCommentRepository::new(pool);
        let mut comments: HashMap<String, Vec<ConversationComment>> = Default::default();
        let test_uuid = uuid::Uuid::new_v4();
        let (comment_id, _) = test_uuid.as_u64_pair();
        comments.insert(
            test_uuid.to_string(),
            vec![ConversationComment {
                comment_id: comment_id as i64,
                user_id: 0,
                user_name: "test_user".to_string(),
                time: Utc::now(),
                channel_id: 0,
                comment: "This is a test comment".to_string(),
            }],
        );
        repo.add_comments(comments).await.unwrap();
        let found = repo.get_latest_from_channel(0).await.unwrap();
        println!("{:?}", found);
    }
}
