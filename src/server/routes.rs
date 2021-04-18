use super::{endpoints, middlewares, util::PercentDecoded};
use crate::state::StateHandle;
use warp::{Filter, Rejection, Reply};

pub fn routes(state: &StateHandle) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    index()
        .or(resource_editor())
        .or(resources())
        .or(file())
        .or(items(state))
}

fn index() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(|| async { Ok::<_, Rejection>("Market API Server") })
}

fn resource_editor() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("resEditor")
        .and(warp::get())
        .and_then(endpoints::get_resource_editor)
}

fn resources() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_resource = warp::path!(PercentDecoded)
        .and(warp::get())
        .and_then(endpoints::get_resource);

    let get_all_resources = warp::path::end()
        .and(warp::get())
        .and(middlewares::authenticate())
        .and_then(endpoints::get_all_resources);

    let post_resource = warp::path::end()
        .and(warp::post())
        .and(middlewares::authenticate())
        .and(warp::body::json())
        .and_then(endpoints::post_resource);

    let delete_resource = warp::path!(PercentDecoded)
        .and(warp::delete())
        .and(middlewares::authenticate())
        .and_then(endpoints::delete_resource);

    warp::path!("res" / ..).and(
        get_resource
            .or(get_all_resources)
            .or(post_resource)
            .or(delete_resource),
    )
}

fn file() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let get_file = warp::path!(PercentDecoded)
        .and(warp::get())
        .and_then(endpoints::get_file);

    let get_all_files = warp::path::end()
        .and(warp::get())
        .and(middlewares::authenticate())
        .and_then(endpoints::get_all_files);

    let upload_file = warp::path!(PercentDecoded)
        .and(warp::put())
        .and(middlewares::authenticate())
        .and(warp::body::bytes())
        .and_then(endpoints::upload_file);

    let delete_file = warp::path!(PercentDecoded)
        .and(warp::delete())
        .and(middlewares::authenticate())
        .and_then(endpoints::delete_file);

    warp::path!("file" / ..).and(get_file.or(get_all_files).or(upload_file).or(delete_file))
}

fn items(state: &StateHandle) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("all")
        .and(warp::get())
        .and(warp::query())
        .and(with_state(state))
        .and_then(endpoints::get_all_items)
}

fn with_state(
    state: &StateHandle,
) -> impl Filter<Extract = (StateHandle,), Error = std::convert::Infallible> + Clone {
    let state = state.clone();
    warp::any().map(move || state.clone())
}