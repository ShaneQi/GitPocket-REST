extern crate iron;

use self::iron::prelude::*;
use models::*;
use super::get_resp;

pub fn get_repo_tags(req: &mut Request) -> IronResult<Response> {
    get_resp(req, "repo_id", |repo_id| Tag::of_repo(repo_id))
}