// use async_graphql::{Context, InputObject, Object, Result};
// use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};


// use crate::{schema::users, models::User, ToDoError};

// use super::Mutation;




// #[derive(InputObject)]
// pub struct ILogin {
//     pub email: String,
//     pub password: String,
// }

// #[Object]
// impl Mutation {
//     async fn login<'ctx>(&self, ctx: &Context<'ctx>, credentials: ILogin) -> Result<bool> {
//         let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
//         let mut connection = pool.get().await?;

//         let user = users::table
//             .filter(users::email_address.eq(&credentials.email))
//             .first::<User>(&mut connection)
//             .await
//             .map_err(|e| {
//                 log::error!("Failed to fetch user: {}", e);
//                 ToDoError::UserNotFound
//             })?;

//         if user.password != credentials.password {
//             return Err(ToDoError::InvalidCredentials);
//         }

//         Ok(true)
//     }
// }
