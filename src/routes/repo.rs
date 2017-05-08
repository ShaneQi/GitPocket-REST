extern crate iron;
extern crate serde_json;

use self::iron::prelude::*;
use models::*;
use super::get_resp;
use super::post_resp;
use super::delete_resp;

pub fn get_user_repos(req: &mut Request) -> IronResult<Response> {
    get_resp(req, "user_id", |user_id| Repo::of_user(user_id))
}

pub fn post_user_repo(req: &mut Request) -> IronResult<Response> {
    post_resp(req, "user_id", |user_id, body_content| {
        serde_json::from_str::<Repo>(body_content)
            .ok()
            .and_then(|mut repo| repo.post(user_id).ok().and(Some(repo)))
    })
}

pub fn delete_repo(req: &mut Request) -> IronResult<Response> {
    delete_resp(req, "repo_id", |repo_id| Repo::delete(repo_id) )
}