extern crate git_pocket_rest;
extern crate iron;
extern crate router;
extern crate unicase;

use iron::prelude::*;
use iron::Chain;
use iron::AfterMiddleware;
use iron::method::Method::{Options, Post, Get, Delete};
use iron::headers;
use unicase::UniCase;
use git_pocket_rest::routes::router;

struct OptionsHandler;

impl AfterMiddleware for OptionsHandler {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        if req.method == Options {
            res.headers.set(headers::AccessControlAllowOrigin::Any);
            res.headers
                .set(headers::AccessControlAllowMethods(vec![Get, Post, Delete]));
            res.headers
                .set(headers::AccessControlAllowHeaders(vec![UniCase("content-type".to_string())]));
        }
        Ok(res)
    }
}

fn main() {
    let mut chain = Chain::new(router());
    chain.link_after(OptionsHandler);
    Iron::new(chain).http("localhost:3001").unwrap();
}