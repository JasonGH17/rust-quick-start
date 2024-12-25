use crate::{error::Result, ModelManager};
use lib_auth::scheme::AuthScheme;

mod dto {
    use serde::{Deserialize, Serialize};

    use crate::DateTimeLocal;

    #[derive(Debug, Clone, sqlx::FromRow)]
    pub struct User {
        pub id: i32,
        pub creator_id: Option<i32>,
        pub email: String,
        pub password_hash: String,
        pub is_active: bool,
        pub first_login: bool,
        pub created_at: DateTimeLocal,
        pub updated_at: DateTimeLocal,
        pub last_login_at: DateTimeLocal,
    }

    #[derive(Debug, Clone, sqlx::FromRow, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PartialUser {
        pub id: i32,
        pub creator_id: Option<i32>,
        pub email: String,
        pub is_active: bool,
        pub last_login_at: DateTimeLocal,
    }

    impl From<User> for PartialUser {
        fn from(
            User {
                id,
                creator_id,
                email,
                is_active,
                last_login_at,
                first_login: _,
                password_hash: _,
                created_at: _,
                updated_at: _,
            }: User,
        ) -> Self {
            PartialUser {
                id,
                creator_id,
                email,
                is_active,
                last_login_at,
            }
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateUserData {
        pub creator_id: i32,
        pub email: String,
        pub password: String,
    }
}
pub use dto::*;

pub struct UserBmc;

impl UserBmc {
    pub async fn create(mm: &ModelManager, data: CreateUserData) -> Result<i32> {
        let pool = mm.db();

        let hash = lib_auth::scheme::argon::ArgonScheme::hash_password(data.password).await?;

        let (id,) = sqlx::query_as(
            r#"INSERT INTO "user" (email, password_hash, creator_id) VALUES ($1, $2, $3) RETURNING "id""#,
        )
        .bind(data.email)
        .bind(hash)
        .bind(data.creator_id)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    pub async fn find_by_id(mm: &ModelManager, id: i32) -> Result<Option<User>> {
        let pool = mm.db();

        let user = sqlx::query_as(r#"SELECT * FROM "user" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(mm: &ModelManager, email: &str) -> Result<Option<User>> {
        let pool = mm.db();

        let user = sqlx::query_as(r#"SELECT * FROM "user" WHERE "email" = $1"#)
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn find_all(mm: &ModelManager) -> Result<Vec<PartialUser>> {
        let pool = mm.db();

        let users = sqlx::query_as(r#"SELECT * FROM "user""#)
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn set_password_by_id(mm: &ModelManager, id: i32, hash: String) -> Result<()> {
        let pool = mm.db();

        sqlx::query(r#"UPDATE "user" SET "password_hash" = $1 WHERE "id" = $2"#)
            .bind(hash)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn set_activation_by_id(mm: &ModelManager, id: i32, is_active: bool) -> Result<()> {
        let pool = mm.db();

        sqlx::query(r#"UPDATE "user" SET "is_active" = $1 WHERE "id" = $2"#)
            .bind(is_active)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_last_login(mm: &ModelManager, id: i32) -> Result<()> {
        let pool = mm.db();

        sqlx::query(r#"UPDATE "user" SET "last_login_at" = CURRENT_TIMESTAMP WHERE "id" = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_first_login(mm: &ModelManager, id: i32, first_login: bool) -> Result<()> {
        let pool = mm.db();

        sqlx::query(r#"UPDATE "user" SET "first_login" = $1 WHERE "id" = $2"#)
            .bind(first_login)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
