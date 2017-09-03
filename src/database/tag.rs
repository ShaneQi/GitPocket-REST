extern crate rusqlite;

use self::rusqlite::Error;
use super::connection;
use models::*;

impl Tag {
    pub fn of_repo(repo_id: i64) -> Result<Vec<Tag>, Error> {
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

    pub fn post(&mut self, repo_id: i64) -> Result<(), Error> {
        let connection = connection();
        let mut statement = try!(
            connection.prepare("INSERT INTO `tags` (name, repo_id) VALUES (:1, :2);"));
        try!(statement.execute(&[&self.name, &repo_id]));
        Ok(())
    }

    pub fn delete(repo_id: i64, tag_name: &str){
        let connection = connection();
        let mut statement = 
            connection.prepare("DELETE FROM `tags` WHERE repo_id = :1 AND name = :2;").unwrap();
        statement.execute(&[&repo_id, &tag_name.to_string()]).unwrap();
    }
}