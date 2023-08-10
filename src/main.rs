use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use todo::mutation::add_todo::AddTodoMutation;
use todo::mutation::sign_up::AddSignUpMutation;
use todo::{InternalError, Mutation, Query};


async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let request = req.into_inner();
    schema.execute(request).await.into()
}


async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = dotenvy::var("DATABASE_URL").unwrap();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build()?;

    let schema = Schema::build(Query, Mutation(AddSignUpMutation,AddTodoMutation), EmptySubscription)
        .data(pool)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}