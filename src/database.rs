extern crate rusqlite;
extern crate dotenv;

use self::rusqlite::Connection;
use self::rusqlite::Error;
use models::*;

fn connection() -> Connection {
    let db_url = dotenv::var("DATABASE_URL").expect("Couldn't find `DATABASE_URL` from .env file.");
    Connection::open(db_url).expect("Couldn't open database.")
}

impl Repo {
    pub fn of_user(user_id: i32) -> Result<Vec<Repo>, Error> {
        let connection = connection();
        let mut statement =
        try!(connection.prepare("SELECT `id`, `owner`, `name` FROM `repos` WHERE user_id = :1;"));
        let repo_rows = try!(statement.query_map(&[&user_id], |row| {
            Repo {
                id: row.get(0),
                owner: row.get(1),
                name: row.get(2),
                host: None,
                tags: None,
            }
        }));
        let mut repos = Vec::new();
        for repo in repo_rows {
            repos.push(repo.unwrap());
        }
        Ok(repos)
    }
}

impl Host {
    pub fn of_repo(repo_id: i32) -> Result<Host, Error> {
        let connection = connection();
        let mut statement =
        try!(connection.prepare("SELECT `hosts.id`, `hosts.name`, `hosts.url` FROM `hosts`, `repos` WHERE `hosts.id` = `repos.host_id AND `repos.id` = :1;"));
        let row = try!(statement.query_row(&[&repo_id], |row| {
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
            hosts.push(host.unwrap());
        }
        Ok(hosts)
    }
}