//TODO: Implement Proper Error Handling

use std::error::Error;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};
use dotenvy::dotenv;

use crate::{
    core::{app_state::AppState, config::Config, db::init_db},
    features::{
        auth::{
            handler::{hello, login_user, register_user},
            middleware::auth_middleware,
        },
        notes::handler::{
            create_note_handler, delete_note_handler, list_notes_handler,
            read_specific_note_handler, update_note_handler,
        },
    },
};

mod core;
mod features;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = Config::from_env();

    let pool = init_db(&config.database_url).await?;

    let state = AppState {
        db_pool: pool,
        jwt_secret: config.jwt_secret,
    };

    let app = app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to connect to the Server");

    axum::serve(listener, app).await.expect("Error Serving APP");

    Ok(())
}

fn app(state: AppState) -> Router {
    let notes_routes = Router::new()
        .route("/", post(create_note_handler))
        .route("/", get(list_notes_handler))
        .route("/{id}", put(update_note_handler))
        .route("/{id}", delete(delete_note_handler))
        .route("/{id}", get(read_specific_note_handler))
        .route_layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .route(
            "/protected",
            post(hello).route_layer(from_fn_with_state(state.clone(), auth_middleware)),
        )
        .nest("/notes", notes_routes)
        .with_state(state)
}
