#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate hyper;
#[macro_use] extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate router;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate tera;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use iron::prelude::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use router::Router;
use std::env;
use std::sync::Arc;
use tera::Tera;

mod data;
mod routes;
mod services;

#[derive(Clone)]
pub struct Application {
    pool: Pool<ConnectionManager<PgConnection>>,
    tera: Arc<Tera>,
}

impl Application {
    fn new() -> Application {
        dotenv().ok();
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        let tera = compile_templates!("templates/**/*");
        Application {
            pool: pool,
            tera: Arc::new(tera),
        }
    }
}

fn main() {
    let app = Application::new();
    let mut router = Router::new();

    router.get("/wiki/:page", routes::wiki::Index{
        app: app.clone(),
    }, "wiki::index");

    router.get("/wiki/:page/edit", routes::wiki::Edit{
        app: app.clone(),
    }, "wiki::edit");

    println!("Running on localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
