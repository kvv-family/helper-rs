use diesel::prelude::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::jobs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Job {
    pub id: i32,
    pub comment: String,
    pub date_start: String,
    pub time_start: String,
    pub date_end: String,
    pub time_end: String,
}