use axum::{
    routing::{get, post},
    Router,
};

mod account;
mod auth;
mod profile;
mod session;

pub async fn routes() -> Router {
    let webdb = crate::database::DBConf::init().await;
    let main_router = Router::new()
        // session handling routes
        .route("/api/user/logout", post(session::logout))
        .route("/api/session/refresh", post(session::refresh_session))
        // user read routes
        .route("/api/user/:id", get(profile::get_user))
        // user update routes
        .route("/api/email/update", post(account::change_email))
        .route("/api/username/update", post(account::change_username))
        .route("/api/password/update", post(account::change_password))
        .route("/api/password/reset", post(account::reset_password))
        .route("/api/metadata/update", post(account::change_metadata))
        .route("/api/account/deactivate", post(account::deactivate_account))
        .with_state(std::sync::Arc::clone(&webdb));

    main_router.merge(auth::auth_routes(webdb))
    // .layer(tower_http::trace::TraceLayer::new_for_http())
}
