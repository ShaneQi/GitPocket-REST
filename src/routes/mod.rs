extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use self::iron::prelude::*;
use self::iron::status;
use self::iron::headers::{ContentType, AccessControlAllowOrigin};
use self::iron::modifiers::Header;
use self::router::Router;
use self::serde::ser::Serialize;

mod repo;
mod host;
mod tag;

pub fn router() -> Router {
    router!{
        get_user_repos: get "/v1/user/:user_id/repos" => repo::get_user_repos,
        get_repo_host: get "/v1/repo/:repo_id/host" => host::get_repo_host,
        get_repo_tags: get "/v1/repo/:repo_id/tags" => tag::get_repo_tags,
    }
}

fn resp(content: String) -> IronResult<Response> {
    Ok(Response::with((status::Ok,
                       content,
                       Header(ContentType::json()),
                       Header(AccessControlAllowOrigin::Any))))
}

fn resp_err() -> IronResult<Response> {
    Ok(Response::with(status::InternalServerError))
}

fn get_resp<F: FnOnce(i32) -> Result<T, E>, T, E>(req: &mut Request,
                                                  query_name: &str,
                                                  fetch: F)
                                                  -> IronResult<Response>
    where T: Serialize
{
    match req.extensions
              .get::<Router>()
              .and_then(|router| router.find(query_name))
              .and_then(|query| query.parse::<i32>().ok())
              .and_then(|id| fetch(id).ok())
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }

}
