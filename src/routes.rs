extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use models::*;

pub fn router() -> Router {
    router!{
        get_user_repos: get "/v1/user/:user_id/repos" => get_user_repos,
        get_repo_host: get "/v1/repo/:repo_id/host" => get_repo_host,
        get_repo_tags: get "/v1/repo/:repo_id/tags" => get_repo_tags,
    }
}

fn get_user_repos(req: &mut Request) -> IronResult<Response> {
    let ref user_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("user_id")
        .unwrap_or("/");
    let repos = Repo::of_user(user_id.parse::<i32>().unwrap()).unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&repos).unwrap())))
}

fn get_repo_host(req: &mut Request) -> IronResult<Response> {
    let ref repo_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("repo_id")
        .unwrap_or("/");
    let host = Host::of_repo(repo_id.parse::<i32>().unwrap()).unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&host).unwrap())))
}

fn get_repo_tags(req: &mut Request) -> IronResult<Response> {
    let ref repo_id = req.extensions
        .get::<Router>()
        .unwrap()
        .find("repo_id")
        .unwrap_or("/");
    let tags = Tag::of_repo(repo_id.parse::<i32>().unwrap()).unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&tags).unwrap())))
}