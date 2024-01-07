use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UrlEntry {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
}

impl UrlEntry {
    pub fn to_string(&self) -> String {
        format!(
            "Short_url: {}, long_url: {}",
            self.short_url, self.long_url
        )
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::urls)]
pub struct NewUrl<'a> {
    pub short_url: &'a str,
    pub long_url: &'a str,
}
