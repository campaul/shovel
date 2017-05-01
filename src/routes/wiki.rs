use hyper::header::ContentType;
use hyper::mime:: {
    Mime,
    TopLevel,
    SubLevel,
};
use iron::prelude::*;
use iron::Handler;
use iron::status;
use router::Router;
use tera::Context;

use Application;
use services;

fn render(req: &mut Request, app: &Application, template: &str) -> IronResult<Response> {
        let ref slug = req.extensions.get::<Router>().unwrap().find("page").unwrap();
        let pool = itry!(app.pool.get());
        let page = iexpect!(services::wiki::get(&pool, slug));

        let mut context = Context::new();
        context.add("page", &page);
        let body = app.tera.render(template, &context).unwrap();

        let mut resp = Response::with((status::Ok, body));
        resp.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(resp)
}

pub struct IndexGet {
    pub app: Application,
}

impl Handler for IndexGet {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        render(req, &self.app, "wiki/index.html")
    }
}

pub struct IndexPost {
    pub app: Application,
}

impl Handler for IndexPost {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        // TODO: Update DB
        render(req, &self.app, "wiki/index.html")
    }
}

pub struct EditGet {
    pub app: Application,
}

impl Handler for EditGet {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        render(req, &self.app, "wiki/edit.html")
    }
}
