extern crate rusqlite;
extern crate dotenv;

use self::rusqlite::Connection;

pub mod repo;
pub mod host;
pub mod tag;

fn connection() -> Connection {
    let db_url = dotenv::var("DATABASE_URL").expect("Couldn't find `DATABASE_URL` from .env file.");
    Connection::open(db_url).expect("Couldn't open database.")
}
