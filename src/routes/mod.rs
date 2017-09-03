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
        post_user_repo: post "/v1/user/:user_id/repos" => repo::post_user_repo,
        delete_repo: delete "v1/repo/:repo_id" => repo::delete_repo,

        get_hosts: get "/v1/hosts" => host::get_hosts,
        get_host: get "/v1/host/:host_id" => host::get_host,
        post_host: post "/v1/hosts" => host::post_host,

        get_repo_tags: get "/v1/repo/:repo_id/tags" => tag::get_repo_tags,
        post_repo_tag: post "/v1/repo/:repo_id/tags" => tag::post_repo_tag,
        delete_repo_tag: delete "v1/repo/:repo_id/tag/:tag_name" => tag::delete_repo_tag,
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

fn get_resp<F: FnOnce(i64) -> Result<T, E>, T, E>(req: &Request,
                                                  query_name: &str,
                                                  get: F)
                                                  -> IronResult<Response>
    where T: Serialize
{
    match req.extensions
              .get::<Router>()
              .and_then(|router| router.find(query_name))
              .and_then(|query| query.parse::<i64>().ok())
              .and_then(|id| get(id).ok())
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }

}

fn get_static_resp<F: FnOnce() -> Result<T, E>, T, E>(get: F) -> IronResult<Response>
    where T: Serialize
{
    match get()
              .ok()
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }

}

fn post_resp<F: FnOnce(i64, &str) -> Option<T>, T>(req: &mut Request,
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
              .and_then(|query| query.parse::<i64>().ok())
              .and_then(|id| post(id, &body_content))
              .and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }
}

fn post_static_resp<F: FnOnce(&str) -> Option<T>, T>(req: &mut Request,
                                                     post: F)
                                                     -> IronResult<Response>
    where T: Serialize
{
    let mut body_content = String::new();
    let _ = req.body.read_to_string(&mut body_content);
    match post(&body_content).and_then(|result| serde_json::to_string(&result).ok()) {
        Some(content) => resp(content),
        None => resp_err(),
    }
}

fn delete_resp<F: FnOnce(i64)>(req: &Request, query_name: &str, handle: F) -> IronResult<Response> {
    match req.extensions
              .get::<Router>()
              .and_then(|router| router.find(query_name))
              .and_then(|query| query.parse::<i64>().ok()) {
        Some(id) => handle(id),
        _ => {}
    }
    resp("".to_string())
}

fn delete_repo_tag_resp<F: FnOnce(i64, &str)>(req: &Request, handle: F) -> IronResult<Response> {
    let repo_id = req.extensions
        .get::<Router>()
        .and_then(|router| router.find("repo_id"))
        .and_then(|query| query.parse::<i64>().ok()).unwrap();
    let tag_name = req.extensions
        .get::<Router>()
        .and_then(|router| router.find("tag_name")).unwrap();
    handle(repo_id, tag_name);
    resp("".to_string())
}