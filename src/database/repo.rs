extern crate rusqlite;

use self::rusqlite::Error;
use super::connection;
use models::*;

impl Repo {
    pub fn of_user(user_id: i64) -> Result<Vec<Repo>, Error> {
        let connection = connection();
        let mut statement =
        try!(connection.prepare("SELECT `id`, `owner`, `name`, `host_id` FROM `repos` WHERE `user_id` = :1;"));
        let repo_rows = try!(statement.query_map(&[&user_id], |row| {
            Repo {
                id: row.get(0),
                owner: row.get(1),
                name: row.get(2),
                host_id: row.get(3),
            }
        }));
        let mut repos = Vec::new();
        for row in repo_rows {
            if let Ok(repo) = row {
                repos.push(repo);
            }
        }
        Ok(repos)
    }

    pub fn post(&mut self, user_id: i64) -> Result<(), Error> {
        let connection = connection();
        let mut statement = try!(connection
            .prepare("INSERT INTO `repos` (name, owner, user_id, host_id) VALUES (:1, :2, :3, :4);"));
        try!(statement.execute(&[&self.name, &self.owner, &user_id, &self.host_id]));
        let repo_id = connection.last_insert_rowid();
        self.id = Some(repo_id);
        Ok(())
    }
    
    pub fn delete(repo_id: i64) {
        let connection = connection();
        let delete_repo_statement = connection.prepare("DELETE FROM `repos` WHERE id = :1;");
        delete_repo_statement.and_then(|mut statm| statm.execute(&[&repo_id])).unwrap();
        let delte_tags_statement = connection.prepare("DELETE FROM `tags` WHERE repo_id = :1;");
        delte_tags_statement.and_then(|mut statm| statm.execute(&[&repo_id])).unwrap();
    }
}
