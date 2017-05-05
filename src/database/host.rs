extern crate rusqlite;

use self::rusqlite::Error;
use super::connection;
use models::*;

impl Host {
    pub fn of_repo(repo_id: i32) -> Result<Host, Error> {
        let connection = connection();
        let mut statement =
        try!(connection.prepare("SELECT hosts.id, hosts.name, hosts.url FROM `hosts`, `repos` WHERE hosts.id = repos.host_id AND repos.id = :1;"));
        let row = try!(statement.query_row(&[&repo_id], |row| {
            Host {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
            }
        }));
        Ok(row)
    }
}

