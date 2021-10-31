use axum::{
    extract::Extension,
    handler::{get, post},
    http::StatusCode,
    response::IntoResponse,
    AddExtensionLayer, Json, Router,
};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use serde::Serialize;

#[tokio::main]
async fn main() {
    let messages = Mutex::new(Messages { msg: Vec::new() });

    let app_state = Arc::new(messages);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/msg", post(create_message).get(get_messages))
        .layer(AddExtensionLayer::new(app_state));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_message(msg: String, state: Extension<Arc<Mutex<Messages>>>) -> impl IntoResponse {
    let mut messages = state.lock().unwrap();
    messages.msg.push(msg.clone());
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(msg))
}

async fn get_messages(state: Extension<Arc<Mutex<Messages>>>) -> impl IntoResponse {
    let response = state.lock().unwrap().clone();
    (StatusCode::OK, Json(response))
}

#[derive(Clone, Serialize)]
struct Messages {
    msg: Vec<String>,
}
