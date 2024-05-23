#[macro_use]
extern crate diesel;

mod schema;
mod models;

use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::users::dsl::*;
use crate::models::User;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn main() {
    dotenv().ok();

    // 从环境变量读取数据库 URL
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL not found");
        std::process::exit(1);
    });
    
    // 创建数据库连接池
    let database_pool = Pool::builder()
        .build(ConnectionManager::new(database_url))
        .expect("Failed to create pool.");
    

    // 获取一个数据库连接
    let mut db_connection = database_pool.get().expect("Failed to get a connection from the pool.");

    // 查询所有用户
    match users.load::<User>(&mut db_connection) {
        Ok(results) => {
            println!("Displaying {} users", results.len());
            for user in results {
                match user.id {
                    Some(user_id) => println!("ID: {}", user_id),  // 使用新的变量名称
                    None => println!("ID: None"),
                }
                println!("Name: {}", user.name);
                println!("Address: {}", user.address);
                println!("Date Created: {}", user.date_created);
                println!("----------\n");
            }
        },
        Err(err) => {
            eprintln!("Error loading users: {}", err);
        },
    }
}