#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate router;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::prelude::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use router::Router;
use std::env;

mod data;
mod routes;
mod services;

#[derive(Clone)]
pub struct Application {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Application {
    fn new() -> Application {
        dotenv().ok();
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        Application {
            pool: pool,
        }
    }
}

fn main() {
    let app = Application::new();
    let mut router = Router::new();

    router.get("/wiki/:page", routes::wiki::Index{
        app: app.clone(),
    }, "wiki::index");

    println!("Running on localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
