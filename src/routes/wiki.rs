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

pub struct Index {
    pub app: Application,
}

impl Handler for Index {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref slug = req.extensions.get::<Router>().unwrap().find("page").unwrap();
        let pool = itry!(self.app.pool.get());
        let page = iexpect!(services::wiki::get(&pool, slug));

        let mut context = Context::new();
        context.add("page", &page);
        let body = self.app.tera.render("wiki/index.html", &context).unwrap();

        let mut resp = Response::with((status::Ok, body));
        resp.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(resp)
    }
}

pub struct Edit {
    pub app: Application,
}

impl Handler for Edit {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref slug = req.extensions.get::<Router>().unwrap().find("page").unwrap();
        let pool = itry!(self.app.pool.get());
        let page = iexpect!(services::wiki::get(&pool, slug));

        let mut context = Context::new();
        context.add("page", &page);
        let body = self.app.tera.render("wiki/edit.html", &context).unwrap();

        let mut resp = Response::with((status::Ok, body));
        resp.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(resp)
    }
}
