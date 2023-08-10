use diesel::{Queryable, Selectable, Insertable};



#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewToDo<'a> {
    pub username: &'a str,
    pub title: &'a str,
    pub completed: bool,
    pub description: Option<&'a str>,
    pub due_date: Option<chrono::NaiveDate>,
    pub completed_date: Option<chrono::NaiveDate>,
}