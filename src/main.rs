use std::env;

use migrator::Migrator;
use sea_orm_migration::MigratorTrait;

mod db;
mod migrator;

#[macro_use]
extern crate rocket;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_sercert: String,
}

impl AppConfig {
    fn new() -> Self {
        Self {
            db_host: std::env::var("DATA_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("DATA_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("DATA_DB_USERNAME").unwrap_or("data".to_string()),
            db_password: std::env::var("DATA_DB_PASSWORD").unwrap_or("20241016".to_string()),
            db_database: std::env::var("DATA_DB_DATABASE").unwrap_or("data".to_string()),
            jwt_sercert: std::env::var("DATA_JWT_SERCERT").unwrap_or("13579".to_string()),
        }
    }
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
async fn rocket() -> _ {
    env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    env::set_var("ROCKET_PORT", "80");

    let config = AppConfig::new();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("[-] 数据库连接失败{}", err),
    };

    match Migrator::up(&db, None).await {
        Ok(_) => (),
        Err(err) => panic!("[-] 数据库迁移失败{}", err),
    }

    // 卸载数据库
    // match Migrator::down(&db, None).await {
    //     Ok(_) => (),
    //     Err(err) => panic!("[-] 数据库迁移失败{}", err),
    // }

    rocket::build().mount("/hello", routes![hello])
}
