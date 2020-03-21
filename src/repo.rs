use std::any::TypeId;

use diesel::expression::NonAggregate;
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::query_builder::{AstPass, QueryFragment, QueryId};
use diesel::sql_types::Text;

use crate::models::Post;

#[derive(QueryId)]
pub enum TextSearch {
    Is(String),
    Contains(String),
}


impl QueryFragment<diesel::pg::Pg> for TextSearch {
    fn walk_ast(&self, mut out: AstPass<diesel::pg::Pg>) -> QueryResult<()> {
        match self {
            TextSearch::Is(str) => out.push_bind_param::<Text, _>(&format!("{}", str))?,
            TextSearch::Contains(str) => out.push_bind_param::<Text, _>(&format!("%{}%", str))?,
        }

        Ok(())
    }
}

impl AppearsOnTable<crate::schema::posts::table> for TextSearch {}

impl Expression for TextSearch { type SqlType = Text; }

impl NonAggregate for TextSearch {}


pub fn find_post(conn: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    let contains_lorem = TextSearch::Contains("lorem".to_owned());
    let is_title = TextSearch::Is("et sequi amet officia.".to_owned());

    posts
        .filter(body.like(contains_lorem))
        .filter(title.eq(is_title))
        .limit(5)
        .load::<Post>(conn)
        .unwrap()
}