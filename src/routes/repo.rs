extern crate iron;

use self::iron::prelude::*;
use models::*;
use super::get_resp;

pub fn get_user_repos(req: &mut Request) -> IronResult<Response> {
    get_resp(req, "user_id", |user_id| Repo::of_user(user_id))
}