use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub struct UserCtx {
    user_id: i64,
}

// tuple of one -> ((),)
pub fn check_auth() -> impl Filter<Extract = (UserCtx,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(HEADER_XAUTH))
        .and_then(|xauth: String| async move {
            // TODO - implement real checking of expiration and signature
            if !xauth.ends_with(".exp.signature") {
                return Err(warp::reject::custom(FailedAuth));
            }

            let user_id = xauth
                .split(".")
                .next()
                .and_then(|value| value.parse::<i64>().ok());

            //turbofish needed because we're in a closure
            match user_id {
                //turbofish needed because we're in a closure
                Some(user_id) => return Ok::<UserCtx, warp::Rejection>(UserCtx { user_id }),
                //turbofish needed because we're in a closure
                None => return Err(warp::reject::custom(FailedAuth)),
            }
            // if let Some(user_id) = xauth
            //     .splitn(2, ".")
            //     .next()
            //     .and_then(|value| value.parse::<i64>().ok())
            // {
            //     Ok::<UserCtx, warp::Rejection>(UserCtx { user_id })
            // } else {
            //     Err(warp::reject::custom(FailedAuth))
            // }
        })
}

#[derive(Debug)]
pub struct FailedAuth;

impl warp::reject::Reject for FailedAuth {}
