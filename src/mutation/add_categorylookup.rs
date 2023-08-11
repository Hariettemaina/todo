use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use async_graphql::{Context, InputObject, Object, Result};
use crate::{models::NewCategoryLookup, schema::category_lookup, ToDoError};


#[derive(InputObject)]
pub struct ICategorylookup {
    pub name: String,
    pub description: Option<String>,
}
impl<'a> From<&'a ICategorylookup> for NewCategoryLookup<'a> {
    fn from(input: &'a ICategorylookup) -> Self {
        Self {
            name:&input.name,
            description:input.description.as_deref()
        }
    }
}

#[derive(Default)]
pub struct AddCategoryLookupMutation;
#[Object]
impl AddCategoryLookupMutation {
    async fn sign_up<'ctx>(&self, ctx: &Context<'ctx>, credentials: ICategorylookup) -> Result<bool> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let new_categorylkp: NewCategoryLookup = (&credentials).into();

        diesel::insert_into(category_lookup::table)
            .values(new_categorylkp)
            .execute(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                ToDoError::UserAccountAlreadyExists
            })?;

        Ok(true)
    }
    
}