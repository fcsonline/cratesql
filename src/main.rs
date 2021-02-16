#![deny(warnings)]
use std::{collections::HashMap, env};

#[macro_use]
extern crate diesel;

mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use dotenv::dotenv;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

#[derive(Clone, GraphQLObject, Queryable)]
#[graphql(description = "A Rust package")]
pub struct Crate {
  id: i32,
  name: String,
  updated_at: NaiveDateTime,
  created_at: NaiveDateTime,
  downloads: i32,
  description: Option<String>,
  homepage: Option<String>,
  documentation: Option<String>,
  readme: Option<String>,
  repository: Option<String>,
  max_upload_size: Option<i32>,
}

#[derive(Clone, GraphQLObject, Queryable)]
#[graphql(description = "A keyword")]
pub struct Keyword {
  id: i32,
  keyword: String,
  crates_cnt: i32,
  created_at: NaiveDateTime,
}

#[derive(Default, Clone)]
pub struct Database {
    ///this could be a database connection
    crates: HashMap<i32, Crate>,
}

impl Database {
    pub fn new() -> Database {
        let crates = HashMap::new();

        Database { crates }
    }
    pub fn get_crate(&self, id: &i32) -> Option<&Crate> {
        self.crates.get(id)
    }
}

impl juniper::Context for Database {}

struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn apiVersion() -> String {
        "1.0".to_string()
    }

    fn dumpDate() -> String {
        "2021-02-01".to_string()
    }

    #[graphql(
      name = "crate",
      arguments(
        id(description = "id of the crate"),
        name(description = "name of the crate")
      )
    )]
    fn cratez(database: &Database, id: i32, _name: Option<String>) -> Option<&Crate> {
        database.get_crate(&id)
    }

    #[graphql(name = "crates", arguments())]
    fn crates(_database: &Database) -> Vec<Crate> {
        use crate::schema::crates::dsl::*;
        let connection = establish_connection();

        crates
          .limit(100)
          .load::<Crate>(&connection)
          .expect("Error loading crates")
    }

    #[graphql(name = "keywords", arguments())]
    fn keywords(_database: &Database) -> Vec<Keyword> {
        use crate::schema::keywords::dsl::*;
        let connection = establish_connection();

        keywords
          .limit(100)
          .load::<Keyword>(&connection)
          .expect("Error loading keywords")
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new();
    graphql_handler(&schema, &context, req, payload).await
}

fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .data(schema())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(
                web::resource("/graphgl")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
