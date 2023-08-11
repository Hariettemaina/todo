use diesel::{Queryable, Insertable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::category_lookup)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategoryLookup<'a> {
    pub name:&'a str,
    pub description: Option<&'a str>,
}