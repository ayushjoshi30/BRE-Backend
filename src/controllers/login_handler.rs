use crate::auth::auth::*;
use crate::error;
use std::sync::Arc;
use entity::g_appusers as users;
use crate::models::login_model::{LoginRequest, LoginResponse};
use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter};
use warp::{reject, reply, Rejection, Reply};
use entity::g_appusers::Entity as UserEntity;
use sha2::{Digest, Sha256};

// Define the result type for web responses
pub type WebResult<T> = std::result::Result<T, Rejection>;

// Define the result type for application logic
pub type Result<T> = std::result::Result<T, error::Error>;

// Handler function for login requests
pub async fn login_handler(
    body: LoginRequest,
    db_pool: Arc<DatabaseConnection>
) -> WebResult<impl Reply> {
    match authenticate_user(body.clone(), db_pool).await {
        Ok(true) => {
            // Assuming `create_jwt` generates a JWT token
            let token = create_jwt(body.username)
                .map_err(|_| reject::custom(error::Error::JWTTokenError))?;
            
            Ok(reply::json(&LoginResponse { token }))
        },
        Ok(false) => Err(reject::custom(error::Error::WrongCredentialsError)), // User not authenticated
        Err(e) => Err(reject::custom(e)), // Handle specific errors
    }
}

// Function to authenticate a user
pub async fn authenticate_user(
    body: LoginRequest,
    db_pool: Arc<DatabaseConnection>,
) -> Result<bool> {
    // Hash the password
    let mut hasher = Sha256::new();
    hasher.update(body.pw.as_bytes());
    let hashed_password = format!("{:x}", hasher.finalize());

    // Fetch user from database
    match UserEntity::find()
        .filter(users::Column::UserName.eq(body.username))
        .one(&*db_pool)
        .await
    {
        Ok(Some(user)) => {
            if user.password == hashed_password {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Ok(None) => Err(error::Error::ResourceNotFound),
        Err(_) => Err(error::Error::DatabaseError),
    }
}
