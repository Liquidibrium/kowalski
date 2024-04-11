use crate::entities::user::UserEntity;
use sqlx::PgPool;

pub async fn get_user_by_email(
    db: &PgPool,
    email: &String,
) -> Result<Option<UserEntity>, sqlx::Error> {
    let user = sqlx::query_as!(UserEntity,"SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn create_user(db: &PgPool, user: UserEntity) -> Result<UserEntity, sqlx::Error> {
    let user = sqlx::query_as!(UserEntity,
        r#"
        INSERT INTO users (id, created_at, updated_at, first_name, last_name, email, password, status, provider)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
        user.id,
        user.created_at,
        user.updated_at,
        user.first_name,
        user.last_name,
        user.email,
        user.password,
        user.status.to_string(),
        user.provider.to_string(),
        )
        .fetch_one(db)
        .await?;
    Ok(user)
}
