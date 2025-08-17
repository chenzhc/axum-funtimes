use std::{net::SocketAddr};

use axum::{
    http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/rootInfo", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 6000
    let addr = SocketAddr::from(([0,0,0,0], 6000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listeing on {}", addr);

    let (shutdown, rx) = tokio::sync::oneshot::channel::<()>();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
          // this future
          let _ = rx.await;
          
          tracing::info!("shutdow the server...");
        })
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::info!("get root / ");
    return "Hello, World!";
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    // Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    // params.get("myparam");

    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // panic!("test");

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    return (StatusCode::CREATED, Json(user));
    // return Json(user)
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}