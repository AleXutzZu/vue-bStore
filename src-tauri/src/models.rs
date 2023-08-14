use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub status: Option<String>,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub status: Option<&'a str>,
    pub language: &'a str,
}