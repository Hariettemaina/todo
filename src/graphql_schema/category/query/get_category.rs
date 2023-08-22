use crate::{models::Category, schema::category, ToDoError};
use async_graphql::{Context, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

#[derive(Default)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    pub async fn get_category<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Category>> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let mycategory = category::table
            .load::<Category>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to load your categories: {}", e);
                ToDoError::UserNotFound
            })?;

        Ok(mycategory)
    }
}
