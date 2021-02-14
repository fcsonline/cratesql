#![deny(warnings)]

use std::{collections::HashMap, env};

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

#[derive(Clone, GraphQLObject)]
#[graphql(description = "A Rust package")]
pub struct Crate {
  id: i32,
  name: String,
}

impl Crate {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }
}

#[derive(Default, Clone)]
pub struct Database {
    ///this could be a database connection
    crates: HashMap<i32, Crate>,
}

impl Database {
    pub fn new() -> Database {
        let mut crates = HashMap::new();
        crates.insert(
            1,
            Crate {
                id: 1,
                name: "serde".to_string(),
            },
        );
        crates.insert(
            2,
            Crate {
                id: 2,
                name: "juniper".to_string(),
            },
        );

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
