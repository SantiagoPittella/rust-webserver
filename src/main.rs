use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
  extract::{Json, Extension, Path},
  response::IntoResponse,
  routing::{get, post},
  Router,
};

use serde::{Deserialize, Serialize};

// Structs
#[derive(Clone, Debug, Serialize, Default)]
struct State {
  users: HashMap<u8, User>,
  items: HashMap<u8, Item>,
}

impl State {
  fn new(mut users_hashmap: HashMap<u8, User>, mut items_hashmap: HashMap<u8, Item>) -> Self {
    State { users: users_hashmap, items: items_hashmap}
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

type SharedState = Arc<Mutex<State>>;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  // not the greatest idea, but i will save items and users in this hash maps
  // let mut state = State::new();
  let mut state: SharedState = Arc::new(
    Mutex::new(
      State::new(HashMap::new(), HashMap::new())
    )
  );

  // build our application with a single route
  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/users/:user_id", get(get_user))
    .route("/users", post(create_user))
    .route("/users", get(list_users))
    .route("/items/:item_id", get(get_item))
    .route("/items", post(create_item))
    .route("/items", get(list_items))
    .layer(Extension(state));

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}

// Handlers
async fn get_state(Extension(state): Extension<SharedState>) -> impl IntoResponse {
  // Json(&state)
}

async fn list_users(Extension(state): Extension<SharedState>) -> impl IntoResponse {
  Json(state.lock().unwrap().users.clone())
}

async fn get_user(Path(user_id): Path<u8>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
  let db = &state.lock().unwrap().users;
  Json(db.get(&user_id).unwrap().clone())
}
async fn create_user(Json(user_rq): Json<User>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
  let user: User =  User::new(user_rq.username.to_string(), user_rq.age, user_rq.id);
  &mut state.lock().unwrap().users.insert(user_rq.id, user.clone());
  Json(user)
}

async fn get_item(Path(item_id): Path<u8>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
  let db = &state.lock().unwrap().items;
  Json(db.get(&item_id).unwrap().clone())
}
async fn create_item(Json(item_rq): Json<Item>, Extension(mut state): Extension<SharedState>) -> impl IntoResponse {
  let item: Item = Item::new(item_rq.name.to_string(), User::new(item_rq.owner.username.to_string(), item_rq.owner.age, item_rq.owner.id), item_rq.id);
  &mut state.lock().unwrap().items.insert(item_rq.id, item.clone());
  Json(item)
}

async fn list_items(Extension(state): Extension<SharedState>) -> impl IntoResponse {
  Json(state.lock().unwrap().items.clone())
}
