mod account;
mod auth;
mod session;

use axum::{
    http::HeaderMap,
    routing::{get, post},
    Router,
};

pub async fn routes() -> Router {
    let webdb = crate::database::WebDB::init().await;
    let main_router = Router::new()
        .route("/", get(home_page))
        // session handling routes
        .route("/api/user/logout", post(session::logout))
        .route("/api/session/refresh", post(session::refresh_session))
        // user update routes
        .route("/api/email/verify", post(auth::verify_email))
        .route("/api/email/update", post(account::change_email))
        .route("/api/username/update", post(account::change_username))
        .route("/api/password/update", post(account::change_password))
        .route("/api/password/reset", post(account::reset_password))
        .route("/api/metadata/update", post(account::change_metadata))
        .route("/api/account/delete", post(account::delete_account))
        .with_state(std::sync::Arc::clone(&webdb));

    main_router.merge(auth::auth_routes(webdb))
    // .layer(tower_http::trace::TraceLayer::new_for_http())
}

pub async fn home_page(
    axum::extract::connect_info::ConnectInfo(conn_info): axum::extract::connect_info::ConnectInfo<
        crate::utils::ClientConnInfo,
    >,
    headers_map: HeaderMap,
) -> String {
    if let Ok(true) = crate::utils::jwt::check_token(&headers_map, conn_info.ip()) {
        "good".to_string()
    } else {
        "bad".to_string()
    }
}
