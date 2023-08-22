use async_graphql::{Context, InputObject, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{models::NewCategory, schema::category, ToDoError};

#[derive(InputObject)]
pub struct ICategory {
    pub todo_id: i32,
    pub category_id: i32,
}
impl<'a> From<&'a ICategory> for NewCategory<'a> {
    fn from(input: &'a ICategory) -> Self {
        Self {
            todo_id: &input.todo_id,
            category_id: &input.category_id,
        }
    }
}

#[derive(Default)]
pub struct AddCategoryMutation;
#[Object]
impl AddCategoryMutation {
    pub async fn addcart<'ctx>(&self, ctx: &Context<'ctx>, credentials: ICategory) -> Result<bool> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let new_user: NewCategory = (&credentials).into();

        diesel::insert_into(category::table)
            .values(new_user)
            .execute(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                ToDoError::UserAccountAlreadyExists
            })?;

        Ok(true)
    }
}
