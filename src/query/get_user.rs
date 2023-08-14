use async_graphql::{Context, Object, Result, SimpleObject};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{models::User, schema::users ,ToDoError};


#[derive(SimpleObject)]
pub struct Myuser {
    username: String,
    email: String,
    //password: String,
}
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn user<'ctx>(&self, ctx: &Context<'ctx>, email: String) -> Result<Myuser> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;
    
        let myuser = users::table
            .filter(users::email_address.eq(&email))
            .first::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to fetch user: {}", e);
                ToDoError::UserNotFound
            })?;
    // map the fields from the User object to the corresponding fields 
    //in the Myuser struct and return the converted Myuser object.
        let myuser = Myuser {
            username: myuser.username,
            email: myuser.email_address,
            //password: myuser.password,
        };
    
        Ok(myuser)
    }
}