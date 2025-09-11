// Database层 - 数据模型
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// Repository层 - 数据访问
pub struct UserRepository {
    pool: sqlx::PgPool,
}

impl UserRepository {
    pub async fn find_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create(&self, name: &str, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
            .bind(name)
            .bind(email)
            .fetch_one(&self.pool)
            .await
    }
}

// Service层 - 业务逻辑
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub async fn get_user(&self, id: i32) -> Result<User, String> {
        self.repo.find_by_id(id).await
            .map_err(|_| "User not found".to_string())
    }

    pub async fn create_user(&self, name: String, email: String) -> Result<User, String> {
        if !email.contains('@') {
            return Err("Invalid email".to_string());
        }
        
        self.repo.create(&name, &email).await
            .map_err(|_| "Failed to create user".to_string())
    }
}

// Handler层 - HTTP处理
use axum::{Json, extract::Path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    name: String,
    email: String,
}

pub async fn get_user(
    Path(id): Path<i32>,
    service: UserService,
) -> Result<Json<UserResponse>, String> {
    let user = service.get_user(id).await?;
    Ok(Json(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
}

pub async fn create_user(
    Json(req): Json<CreateUserRequest>,
    service: UserService,
) -> Result<Json<UserResponse>, String> {
    let user = service.create_user(req.name, req.email).await?;
    Ok(Json(UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
}
