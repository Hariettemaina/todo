use diesel::{Queryable, Selectable, Insertable};
use crate::schema::category;

#[derive(Queryable, Selectable,Insertable)]
#[diesel(table_name = category)]
pub struct NewCategory<'a> {
    pub todo_id: &'a i32,
    pub category_id: &'a i32,
}
