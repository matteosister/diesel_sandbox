extern crate diesel;
extern crate diesel_sandbox;


use diesel_sandbox::*;
use diesel_sandbox::repo;

fn main() {
    let connection = establish_connection();

    let results = repo::find_post(&connection);

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}