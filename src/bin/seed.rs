extern crate diesel;
extern crate diesel_sandbox;

use fake::faker::lorem::en::*;
use fake::Fake;

use self::diesel_sandbox::*;

fn main() {
    let connection = establish_connection();

    for _ in 1..100 {
        create_post(
            &connection,
            Sentence(3..5).fake::<String>().as_str(),
            Paragraph(5..10).fake::<String>().as_str(),
        );
    }
}
