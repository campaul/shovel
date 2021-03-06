extern crate aurelius;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate hyper;
#[macro_use] extern crate iron;
extern crate params;
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
    dotenv().ok();

    let app = Application::new();
    let mut router = Router::new();

    router.get("/wiki/:page", routes::wiki::IndexGet{
        app: app.clone(),
    }, "wiki::index::get");

    router.post("/wiki/:page", routes::wiki::IndexPost{
        app: app.clone(),
    }, "wiki::index::post");

    router.get("/wiki/:page/edit", routes::wiki::EditGet{
        app: app.clone(),
    }, "wiki::edit::get");

    println!("Shovel running...");
    Iron::new(router).http(env::var("SOCKET").unwrap()).unwrap();
}
