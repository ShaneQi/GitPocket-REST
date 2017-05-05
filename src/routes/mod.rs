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
use std::io::Read;

mod repo;
mod host;
mod tag;

pub fn router() -> Router {
    router!{
        get_user_repos: get "/v1/user/:user_id/repos" => repo::get_user_repos,
        post_user_repos: post "/v1/user/:user_id/repo" => repo::post_user_repo,
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

fn get_resp<F: FnOnce(i32) -> Result<T, E>, T, E>(req: &Request,
                                                  query_name: &str,
                                                  get: F)
                                                  -> IronResult<Response>
    where T: Serialize
{
    match req.extensions
              .get::<Router>()
              .and_then(|router| router.find(query_name))
              .and_then(|query| query.parse::<i32>().ok())
              .and_then(|id| get(id).ok())
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }

}
fn post_resp<F: FnOnce(i32, &str) -> Option<T>, T>(req: &mut Request,
                                                   query_name: &str,
                                                   post: F)
                                                   -> IronResult<Response>
    where T: Serialize
{
    let mut body_content = String::new();
    let _ = req.body.read_to_string(&mut body_content);
    match req.extensions
              .get::<Router>()
              .and_then(|router| router.find(query_name))
              .and_then(|query| query.parse::<i32>().ok())
              .and_then(|id| post(id, &body_content))
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }
}