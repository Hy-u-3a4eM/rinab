use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        not_found, send, health_checker, login,
        logout, refresh_access_token, register,
    },
    auth::auth,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/health_checker", get(health_checker))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", get(refresh_access_token))
        .route(
            "/logout",
            get(logout)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/me",
            get(send)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/send",
            post(send)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state);
    router.fallback(not_found)
}
