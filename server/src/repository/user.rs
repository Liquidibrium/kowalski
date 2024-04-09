use crate::entities::user::UserEntity;
use sqlx::PgPool;

pub async fn get_user_by_email(
    db: &PgPool,
    email: &String,
) -> Result<Option<UserEntity>, sqlx::Error> {
    let user = sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(db)
        .await?;
    Ok(user)
}

pub async fn create_user(db: &PgPool, user: UserEntity) -> Result<UserEntity, sqlx::Error> {
    let user = sqlx::query_as::<_, UserEntity>(
        r#"
        INSERT INTO users (id, created_at, updated_at, first_name, last_name, email, password, status, provider)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
        .bind(user.id)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.status)
        .bind(&user.provider)
        .fetch_one(db)
        .await?;
    Ok(user)
}
