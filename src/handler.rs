use std::sync::Arc;

use axum::{
    extract::State, Json
};
use jsonwebtoken::{encode, Header};
use crate::{AppState, AuthBody, AuthError, Authlogin, Claims, User, KEYS};

pub async fn login(State(data): State<Arc<AppState>>, Json(login): Json<Authlogin>) -> Result<Json<AuthBody>, AuthError> {
    // Получаем пользователя по имени из базы данных
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password FROM users WHERE username = $1",
    )
        .bind(&login.username)
        .fetch_one(&data.db_pool)
        .await
        .map_err(AuthError::Database)?;
    // Check if the user sent the credentials
    if login.username.is_empty() || login.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if login.username != user.username || login.password != user.password {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the logind token
    Ok(Json(AuthBody::new(token)))
}

pub async fn get_user(State(data): State<Arc<AppState>>, claims: Claims) -> Result<String, AuthError> {
    // Send the user data to the user
    Ok(format!(
        "Welcome to the user area :)\nYour data:\n{claims}",
    ))
}
