extern crate iron;
extern crate serde_json;

use self::iron::prelude::*;
use models::*;
use super::get_resp;
use super::post_resp;
use super::delete_repo_tag_resp;

pub fn get_repo_tags(req: &mut Request) -> IronResult<Response> {
    get_resp(req, "repo_id", |repo_id| Tag::of_repo(repo_id))
}

pub fn post_repo_tag(req: &mut Request) -> IronResult<Response> {
    post_resp(req, "repo_id", |repo_id, body_content| {
        serde_json::from_str::<Tag>(body_content)
            .ok()
            .and_then(|mut tag| tag.post(repo_id).ok().and(Some(tag)))
    })
}

pub fn delete_repo_tag(req: &mut Request) -> IronResult<Response> {
    delete_repo_tag_resp(req, |repo_id, tag_name| Tag::delete(repo_id, tag_name))
}