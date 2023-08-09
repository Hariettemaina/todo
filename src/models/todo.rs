use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
pub struct ToDo {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub completed: bool,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub completed_date: Option<chrono::NaiveDateTime>,
}