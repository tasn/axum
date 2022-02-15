//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

// Just using this file for manual testing. Will be cleaned up before an eventual merge

use axum::{response::IntoResponse, Router};
use axum_extra::routing::{Any, Get, OneOf, Post, RouterExt, TypedPath};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .typed_route(users_index)
        .typed_route(users_create)
        .typed_route(users_show)
        .typed_route(users_edit);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(TypedPath)]
#[typed_path("/users")]
struct UsersCollection;

#[derive(Deserialize, TypedPath)]
#[typed_path("/users/:id")]
struct UsersMember {
    id: u32,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/users/:id/edit")]
struct UsersEdit(u32);

async fn users_index(_: Get, _: UsersCollection) -> impl IntoResponse {
    "users#index"
}

async fn users_create(_: Post, _: UsersCollection, _payload: String) -> impl IntoResponse {
    "users#create"
}

async fn users_show(_: Any, UsersMember { id }: UsersMember) -> impl IntoResponse {
    format!("users#show: {}", id)
}

async fn users_edit(_: OneOf<(Get, Post)>, UsersEdit(id): UsersEdit) -> impl IntoResponse {
    format!("users#edit: {}", id)
}
