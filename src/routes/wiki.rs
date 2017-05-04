use aurelius;
use hyper::header::ContentType;
use hyper::mime:: {
    Mime,
    TopLevel,
    SubLevel,
};
use iron::prelude::*;
use iron::Handler;
use iron::status;
use params::{
    Params,
    Value,
};
use router::Router;
use tera::Context;

use Application;
use services;

fn render(req: &mut Request, app: &Application, template: &str, raw: bool) -> IronResult<Response> {
        let ref slug = req.extensions.get::<Router>().unwrap().find("page").unwrap();
        let pool = itry!(app.pool.get());
        let mut page = iexpect!(services::wiki::get(&pool, slug));

        if !raw {
            page.body = aurelius::markdown::to_html(page.body.as_ref());
        }

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
        render(req, &self.app, "wiki/index.html", false)
    }
}

pub struct IndexPost {
    pub app: Application,
}

impl Handler for IndexPost {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let map = req.get::<Params>().unwrap();
        let title = iexpect!(match map.find(&["title"]) {
            Some(&Value::String(ref title)) => Some(title),
            _ => None,
        });
        let body = iexpect!(match map.find(&["body"]) {
            Some(&Value::String(ref body)) => Some(body),
            _ => None,
        });

        {
            let slug = req.extensions.get::<Router>().unwrap().find("page").unwrap();
            let pool = itry!(self.app.pool.get());
            services::wiki::update(&pool, slug, title, body);
        }

        render(req, &self.app, "wiki/index.html", false)
    }
}

pub struct EditGet {
    pub app: Application,
}

impl Handler for EditGet {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        render(req, &self.app, "wiki/edit.html", true)
    }
}
