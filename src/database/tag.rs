extern crate rusqlite;

use self::rusqlite::Error;
use super::connection;
use models::*;

impl Tag {
    pub fn of_repo(repo_id: i32) -> Result<Vec<Tag>, Error> {
        let connection = connection();
        let mut statement =
            try!(connection.prepare("SELECT tags.name FROM `tags` WHERE tags.repo_id = :1;"));
        let tag_rows = try!(statement.query_map(&[&repo_id], |row| Tag { name: row.get(0) }));
        let mut tags = Vec::new();
        for row in tag_rows {
            if let Ok(tag) = row {
                tags.push(tag);
            }
        }
        Ok(tags)
    }
}