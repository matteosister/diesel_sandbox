extern crate diesel;
extern crate diesel_sandbox;

use std::env::args;

use self::diesel::prelude::*;
use self::diesel_sandbox::*;

fn main() {
    use diesel_sandbox::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
