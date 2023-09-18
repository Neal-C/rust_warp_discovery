use serde_json::{json, Value};
use warp::{reply, Filter};

use crate::security::{check_auth, UserCtx};

pub fn todos() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");
    //LIST todos
    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(check_auth())
        .and_then(todo_list);

    let get = todos_base
        .and(warp::get())
        .and(warp::path::param())
        .and(check_auth())
        .and_then(todo_get);

    let create = todos_base
        .and(warp::post())
        .and(warp::body::json())
        .and(check_auth())
        .and_then(todo_create);

    list.or(get).or(create)
}

// synchronous -> .map with a closure
// asynchronous -> .and_then -> an async block (until async close are stabilized)

// TODO - return my own struct that implements warp::reply::Json
async fn todo_list(_user_ctx: UserCtx) -> Result<reply::Json, warp::Rejection> {
    // TODO - get from Database
    let todos = json!([
        {"id":1 , "title": "todo 1"},
        {"id":2, "title": "todo 2"},
    ]);

    let todos = warp::reply::json(&todos);

    Ok::<reply::Json, warp::Rejection>(todos)
}

async fn todo_get(id: i64, _user_ctx: UserCtx) -> Result<reply::Json, warp::Rejection> {
    // TODO - get from Database
    let todo = json!({
        "id": id, "title": format!("todo {}", id)
    });

    let todo = warp::reply::json(&todo);
    //turbofish syntax optional, because Rust has all the typing information from function signature for the compiler
    Ok(todo)
}

async fn todo_create(payload: Value, _user_ctx: UserCtx) -> Result<reply::Json, warp::Rejection> {
    let todo = payload;

    let todo = warp::reply::json(&todo);

    Ok(todo)
}
