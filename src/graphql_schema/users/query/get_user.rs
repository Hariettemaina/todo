use crate::{models::User, schema::users, ToDoError};
use async_graphql::{Context, Object, Result};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn get_users<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<User>> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let myusers = users::table
            .load::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to fetch user: {}", e);
                ToDoError::UserNotFound
            })?;

        Ok(myusers)
    }
}
