extern crate rusqlite;

use self::rusqlite::Error;
use super::connection;
use models::*;

impl Host {
    pub fn get(host_id: i64) -> Result<Host, Error> {
        let connection = connection();
        let mut statement =
        try!(connection.prepare("SELECT id, name, url FROM `hosts` WHERE hosts.id = :1;"));
        let row = try!(statement.query_row(&[&host_id], |row| {
            Host {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
            }
        }));
        Ok(row)
    }

    pub fn all() -> Result<Vec<Host>, Error> {
        let connection = connection();
        let mut statement = try!(connection.prepare("SELECT `id`, `name`, `url` FROM `hosts`;"));
        let rows = try!(statement.query_map(&[], |row| {
            Host {
                id: row.get(0),
                name: row.get(1),
                url: row.get(2),
            }
        }));
        let mut hosts = Vec::new();
        for host in rows {
            if let Ok(host) = host {
                hosts.push(host);
            }
        }
        Ok(hosts)
    }

    pub fn post(&mut self) -> Result<(), Error> {
        let connection = connection();
        let mut statement = try!(connection
            .prepare("INSERT INTO `hosts` (name, url) VALUES (:1, :2);"));
        try!(statement.execute(&[&self.name, &self.url]));
        let host_id = connection.last_insert_rowid();
        self.id = Some(host_id);
        Ok(())
    }
}
