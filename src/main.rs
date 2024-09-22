// src/main.rs

#[macro_use]
extern crate rocket;

pub mod api;
pub mod db;
pub mod fairings;
pub mod models;
pub mod routes;
pub mod security;

use db::pool::get_pool;
use dotenvy::dotenv;
use fairings::connection_checker::DbConnectionChecker;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use std::env;

#[launch]
async fn rocket() -> _ {
    // Load environment variables from .env file (optional)
    dotenv().ok();

    // Create the database pool
    let pool = get_pool(&env::var("DATABASE_URL").unwrap()).await;

    // Build and return the Rocket instance
    rocket::build()
        .attach(Template::fairing())
        .attach(DbConnectionChecker)
        .manage(pool)
        .mount(
            "/static",
            FileServer::from(env::var("STATIC_PATH").unwrap()),
        )
        .mount(
            "/",
            routes![
                routes::pages::index,
                routes::pages::about,
                routes::pages::login,
                routes::pages::register,
                routes::pages::search,
            ],
        )
        .mount("/api", routes![api::login::login, api::login::logout ,api::register::register])
}
