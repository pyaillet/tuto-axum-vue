use axum::{
    extract::Extension, handler::post, http::StatusCode, response::IntoResponse, routing::BoxRoute,
    AddExtensionLayer, Json, Router,
};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Starting server on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router<BoxRoute> {
    let messages = Mutex::new(Messages {
        content: Vec::new(),
    });

    let app_state = Arc::new(messages);

    // build our application with a route
    Router::new()
        .route("/api/msg", post(create_message).get(get_messages))
        .layer(AddExtensionLayer::new(app_state))
        .boxed()
}

async fn create_message(
    Json(create_message): Json<CreateMessage>,
    state: Extension<Arc<Mutex<Messages>>>,
) -> impl IntoResponse {
    let mut messages = state.lock().unwrap();
    messages.content.push(create_message.msg.clone());
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(create_message))
}

async fn get_messages(state: Extension<Arc<Mutex<Messages>>>) -> impl IntoResponse {
    let response = state.lock().unwrap().clone();
    (StatusCode::OK, Json(response))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct CreateMessage {
    msg: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct Messages {
    content: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{self, Request};
    use hyper::body;
    use std::net::{SocketAddr, TcpListener};

    #[tokio::test]
    async fn testing_endpoints() {
        let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app().into_make_service())
                .await
                .unwrap();
        });
        let client = hyper::Client::new();

        let create_message_origin = CreateMessage {
            msg: "Hello!".to_string(),
        };

        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{}/api/msg", addr))
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&create_message_origin).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("Getting response");

        assert_eq!(response.status(), 201);
        let body = body::to_bytes(response.into_body())
            .await
            .expect("Getting response body");
        let response = String::from_utf8(body.to_vec()).expect("body to string");
        let create_message =
            serde_json::from_str::<CreateMessage>(&response).expect("string to CreateMessage");

        assert_eq!(create_message, create_message_origin);

        let response = client
            .request(
                Request::builder()
                    .uri(format!("http://{}/api/msg", addr))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("Getting response");

        assert_eq!(response.status(), 200);
        let body = body::to_bytes(response.into_body())
            .await
            .expect("Getting response body");
        let response = String::from_utf8(body.to_vec()).expect("body to string");
        let messages = serde_json::from_str::<Messages>(&response).expect("string to Messages");

        let messages_expected = Messages {
            content: vec!["Hello!".to_string()],
        };

        assert_eq!(messages, messages_expected);
    }
}
