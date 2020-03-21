extern crate diesel;
extern crate diesel_sandbox;

use std::env::args;

use self::diesel::prelude::*;
use self::diesel_sandbox::*;
use self::models::Post;

fn main() {
    use diesel_sandbox::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
