use diesel::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Bool;

use crate::models::Post;
use crate::schema::posts;

type DB = diesel::pg::Pg;

#[derive(QueryId)]
pub enum TextSearch {
    Is(String),
    Contains(String),
}


pub fn find_post(conn: &PgConnection) -> Vec<Post> {
    let contains_lorem = TextSearch::Contains("lorem".to_owned());
    let is_title = TextSearch::Is("et sequi amet officia.".to_owned());

    posts::table
        .filter(search_title(contains_lorem))
        //.filter(search_title(is_title))
        .limit(5)
        .load::<Post>(conn)
        .unwrap()
}

fn search_title(search: TextSearch) -> Box<dyn BoxableExpression<posts::table, DB, SqlType=Bool>> {
    match search {
        TextSearch::Is(str) => Box::new(posts::title.eq(str)),
        TextSearch::Contains(str) => Box::new(posts::title.like(format!("%{}%", str))),
    }
}
