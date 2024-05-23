use diesel::{Queryable};
use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
    pub date_created: String,
}