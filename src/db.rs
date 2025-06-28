use crate::models::*;
use crate::error::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn setup_database(db: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            display_name VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ language 'plpgsql';
        "#,
    )
    .execute(db)
    .await?;

    sqlx::query(
        r#"DROP TRIGGER IF EXISTS update_users_updated_at ON users;"#
    )
        .execute(db)
        .await?;

    sqlx::query(
        r#"
    CREATE TRIGGER update_users_updated_at
        BEFORE UPDATE ON users
        FOR EACH ROW
        EXECUTE FUNCTION update_updated_at_column();
    "#
    )
        .execute(db)
        .await?;


    Ok(())
}

pub async fn create_user(db: &PgPool, req: CreateUserRequest) -> Result<User> {
    if req.email.is_empty() || req.password.is_empty() || req.display_name.is_empty() {
        return Err(UserError::Validation("All fields are required".to_string()));
    }
    if req.password.len() < 6 {
        return Err(UserError::Validation("Password must be at least 6 characters".to_string()));
    }
    let existing = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_optional(db)
        .await?;
    if existing.is_some() {
        return Err(UserError::EmailExists);
    }
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;
    let row = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, display_name)
        VALUES ($1, $2, $3)
        RETURNING id, email, display_name, created_at, updated_at
        "#,
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.display_name)
    .fetch_one(db)
    .await?;
    Ok(User {
        id: row.get("id"),
        email: row.get("email"),
        display_name: row.get("display_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn get_user_by_id(db: &PgPool, user_id: Uuid) -> Result<User> {
    let row = sqlx::query(
        "SELECT id, email, display_name, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    match row {
        Some(row) => Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            display_name: row.get("display_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }),
        None => Err(UserError::UserNotFound),
    }
}

pub async fn get_user_by_email(db: &PgPool, email: &str) -> Result<(User, String)> {
    let row = sqlx::query(
        "SELECT id, email, password_hash, display_name, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(db)
    .await?;

    match row {
        Some(row) => Ok((
            User {
                id: row.get("id"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            },
            row.get("password_hash"),
        )),
        None => Err(UserError::UserNotFound),
    }
}

pub async fn update_user(db: &PgPool, user_id: Uuid, req: UpdateUserRequest) -> Result<User> {
    if let Some(ref display_name) = req.display_name {
        if display_name.is_empty() {
            return Err(UserError::Validation("Display name cannot be empty".to_string()));
        }
    }

    let mut query = "UPDATE users SET ".to_string();
    let mut params = Vec::new();
    let mut param_count = 1;

    if let Some(display_name) = req.display_name {
        query.push_str(&format!("display_name = ${}", param_count));
        params.push(display_name);
        param_count += 1;
    }

    if params.is_empty() {
        return get_user_by_id(db, user_id).await;
    }

    query.push_str(&format!(" WHERE id = ${}", param_count));
    query.push_str(" RETURNING id, email, display_name, created_at, updated_at");

    let mut sqlx_query = sqlx::query(&query);
    for param in params {
        sqlx_query = sqlx_query.bind(param);
    }
    sqlx_query = sqlx_query.bind(user_id);

    let row = sqlx_query.fetch_optional(db).await?;

    match row {
        Some(row) => Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            display_name: row.get("display_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }),
        None => Err(UserError::UserNotFound),
    }
}
