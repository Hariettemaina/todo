use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, cookie::Key};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
use todo::mutation::add_categorylookup::AddCategoryLookupMutation;
use todo::mutation::add_todo::AddTodoMutation;
use todo::mutation::sign_up::AddSignUpMutation;
use todo::mutation::add_category::AddCategoryMutation;
use todo::query::get_user::UserQuery;
use todo::query::login::LoginQuery;
use todo::{InternalError, Mutation, Query};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};


async fn index(
    schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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

    let secret_key = Key::generate();

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build()?;

    let schema = Schema::build(Query(LoginQuery,UserQuery), Mutation(AddSignUpMutation,AddTodoMutation,
        AddCategoryMutation,AddCategoryLookupMutation), EmptySubscription)
        .data(pool)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}