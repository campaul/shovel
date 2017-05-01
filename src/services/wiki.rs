use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use data::models;

#[derive(Serialize)]
pub struct Page {
    pub slug: String,
    pub title: String,
    pub body: String,
}

impl Page {
    fn from_db(page: models::Page) -> Page {
        Page {
            slug: page.slug,
            title: page.title,
            body: page.body,
        }
    }
}

pub fn create<'a>(connection: &PgConnection, slug: &'a str, title: &'a str, body: &'a str) -> models::Page {
    use data::schema::pages;

    let new_page = models::NewPage {
        slug: slug,
        title: title,
        body: body,
    };

    diesel::insert(&new_page).into(pages::table)
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn get(connection: &PgConnection, slug: &str) -> Option<Page> {
    use data::schema::pages::dsl;

    let mut selected = dsl::pages
        .filter(dsl::slug.eq(slug))
        .load::<models::Page>(connection)
        .expect("Error loading posts");

    match selected.pop() {
        Some(p) => Some(Page::from_db(p)),
        _ => None,
    }
}
