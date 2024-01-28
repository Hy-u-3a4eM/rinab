use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState,
    handler::{
        get_user,
        login,
    },
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/user", get(get_user))
        .route("/login", post(login))
        .with_state(app_state)
}