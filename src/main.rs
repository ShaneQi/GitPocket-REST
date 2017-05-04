extern crate git_pocket_rest;
extern crate iron;

use iron::prelude::*;
use git_pocket_rest::routes::router;

fn main() {
    Iron::new(router()).http("localhost:3001").unwrap();
}