extern crate iron;
extern crate serde_json;

use self::iron::prelude::*;
use models::*;
use super::get_resp;
use super::get_static_resp;
use super::post_static_resp;

pub fn get_hosts(req: &mut Request) -> IronResult<Response> {
    let _ = req;
    get_static_resp(|| Host::all())
}

pub fn get_host(req: &mut Request) -> IronResult<Response> {
    get_resp(req, "host_id", |host_id| Host::get(host_id))
}

pub fn post_host(req: &mut Request) -> IronResult<Response> {
    post_static_resp(req, |body_content| {
        serde_json::from_str::<Host>(body_content)
            .ok()
            .and_then(|mut host| host.post().ok().and(Some(host)))
    })
}