use async_graphql::{Context, Object, Result, SimpleObject};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{models::User, schema::users ,ToDoError};


#[derive(SimpleObject)]
struct Myuser {
    username: String,
    email: String,
    //password: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn borrow_from_context_data<'ctx>(
        &self,
        ctx: &Context<'ctx>
    ) -> Result<&'ctx String> {
        ctx.data::<String>()
    }
    async fn user<'ctx>(&self, ctx: &Context<'ctx>, email: String) -> Result<Myuser> {
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