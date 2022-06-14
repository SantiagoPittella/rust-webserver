use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

use axum::{
  extract::{Json, Extension, Path},
  response::IntoResponse,
  http::StatusCode,
  routing::{get, post},
  Router,
};

use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Debug, Serialize)]
struct State {
  users: HashMap<u8, User>,
  items: HashMap<u8, Item>,
}

impl State {
  fn new() -> Self {
    State { users: HashMap::new(), items: HashMap::new()}
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
  id: u8,
  username: String,
  age: u8,
}

impl User {
  fn new(username: String, age: u8, id: u8) -> Self {
    User { username: username, age: age, id: id }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Item {
  id: u8,
  name: String,
  owner: User,
}

impl Item {
  fn new(name: String, owner: User, id: u8) -> Self {
    Item { name: name, owner: owner, id: id }
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  // not the greatest idea, but i will save items and users in this hash maps
  let mut state = State::new();
  // let mut state: Arc<State> = Arc::new(State::new());

  // build our application with a single route
  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/users/:user_id", get(get_user))
    .route("/users", post(create_user))
    .route("/items/:item_id", get(get_item))
    .route("/items", post(create_item))
    .route("/state", get(get_state))
    .layer(Extension(state));

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}

// Handlers
async fn get_state(Extension(state): Extension<State>) -> impl IntoResponse {
  Json(state)
}

async fn get_user(Path(user_id): Path<u8>, Extension(state): Extension<State>) -> impl IntoResponse {
  print!("{:?}", state);
  let user: &User = state.users.get(&user_id).unwrap();
  Json(user.clone())
}
async fn create_user(Json(user_rq): Json<User>, Extension(mut state): Extension<State>) -> impl IntoResponse {
  let user: User =  User::new(user_rq.username.to_string(), user_rq.age, user_rq.id);
  state.users.insert(user_rq.id, user.clone());
  Json(user)
}

async fn get_item(Path(item_id): Path<u8>, Extension(state): Extension<State>) -> impl IntoResponse {
  let item: &Item = state.items.get(&item_id).unwrap();
  Json(item.clone())
}
async fn create_item(Json(item_rq): Json<Item>, Extension(mut state): Extension<State>) -> impl IntoResponse {
  let item: Item = Item::new(item_rq.name.to_string(), User::new(item_rq.owner.username.to_string(), item_rq.owner.age, item_rq.owner.id), item_rq.id);
  print!("{:?}", item);
  let item: Item = state.items.insert(item_rq.id, item).unwrap();
  Json(item)
}