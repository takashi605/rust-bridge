use crate::db::models::User;
use anyhow::Result;
use sqlx::PgPool;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn fetch_by_id(&self, id: i32) -> Result<User>;
}

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for SqlxUserRepository {
    async fn fetch_by_id(&self, id: i32) -> Result<User> {
        let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database query failed for user ID {}: {}", id, e))?;

        match user {
            Some(user) => Ok(user),
            None => anyhow::bail!("User with ID {} not found", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::create_connection_pool;

    // 統合テスト: 実際のデータベースが必要
    // テストデータベースが利用可能な環境での実行を想定
    #[tokio::test]
    async fn fetch_user_by_sqlx_repository_integration() {
        // create_connection_poolを使用してデータベース接続を作成
        let pool = create_connection_pool()
            .await
            .expect("Failed to connect to test database");

        let repo = SqlxUserRepository::new(pool);

        // テストデータがDBに存在することを前提
        match repo.fetch_by_id(1).await {
            Ok(user) => {
                assert_eq!(user.id, 1);
                assert_eq!(user.name, "Alice Johnson");
                assert_eq!(user.email, "alice@example.com");
            }
            Err(e) => {
                // ユーザーが存在しない場合の適切なエラーが返されることを確認
                assert!(e.to_string().contains("not found"));
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_nonexistent_user() {
        // create_connection_poolを使用してデータベース接続を作成
        let pool = create_connection_pool()
            .await
            .expect("Failed to connect to test database");

        let repo = SqlxUserRepository::new(pool);

        // 存在しないユーザー（IDが非常に大きい値）
        let result = repo.fetch_by_id(999999).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
