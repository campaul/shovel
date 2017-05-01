use data::schema::pages;

#[derive(Insertable)]
#[table_name="pages"]
pub struct NewPage<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
}
