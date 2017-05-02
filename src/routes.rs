extern crate iron;
extern crate rusqlite;
extern crate router;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;

pub fn router() -> Router {
    router!{
        repos: get "/repos" => repos,
    }
}

fn repos(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "here are your repos")))
}