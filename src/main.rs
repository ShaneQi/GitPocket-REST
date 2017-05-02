extern crate git_pocket_rest;
extern crate diesel;

use self::git_pocket_rest::*;
use self::git_pocket_rest::models::*;
use self::diesel::prelude::*;

fn main() {
    use git_pocket_rest::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .filter(id.eq(1))
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("----------\n");
        println!("{}", user.github_id.unwrap());
    }
}