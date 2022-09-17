// use crate::handlers::*;
use crate::handlers;
// use crate::startup::*;
use crate::startup::AppState;
use warp::{self, Filter};

// use crate::app::AppState;
// use crate::handlers;

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(handlers::index::index)
        .or(warp::path!("api" / "status")
            .and(warp::get())
            .map(handlers::status::status))
        .or(warp::path!("api" / "hello")
            .and(warp::get())
            .and(warp::header("Authorization"))
            .and(with_state(state.clone()))
            .map(handlers::hey))
}

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}