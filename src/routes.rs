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