use uuid::Uuid;

use crate::domain::Repository;

use crate::response::Response;
use crate::startup::*;
use crate::handlers::responses::HelloResponse;

pub async fn hey(greet: Uuid, state: AppState) -> Response {
    let repository = &state.repository;
    let greet = repository.hello(greet)?;//.get_user_by_id(user_id)?;

    let response = HelloResponse::from(greet);
    Ok(warp::reply::json(&response))
}