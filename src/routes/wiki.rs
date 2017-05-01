use iron::prelude::*;
use iron::Handler;
use iron::status;
use router::Router;

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
        Ok(Response::with((status::Ok, page.body)))
    }
}
