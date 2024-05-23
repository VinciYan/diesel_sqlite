use diesel::{Queryable, Insertable};
use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
    pub date_created: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub date_created: &'a str,
}