#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    //    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgresql://fat:ps@localhost/mock".to_owned());
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use actix_web::{web, App, HttpServer, Responder};

fn index(info: web::Path<(String, u32)>) -> impl Responder {
    format!("Hello {}! id:{}", info.0, info.1)
}

mod schema;

mod db;

use crate::schema::doc;

fn main() -> std::io::Result<()> {
    serve()
}

use web::Data;

use actix::prelude::*;
use actix_server::Server;
use actix_http::{HttpService, KeepAlive};
use actix_web::web::{
    HttpRequest,
    HttpResponse,
};

fn serve() -> std::io::Result<()> {
    //    let conn = establish_connection();
    //    HttpServer::new(|| App::new()
    //        .data(conn)
    //        .service(
    //            web::resource("/{name}/{id}/index.html").to(index))
    //    )
    //        .bind("127.0.0.1:8080")?
    //        .run()
    let sys = actix_rt::System::new("docify");
    let db_url = "postgres://fat:ps@localhost/mock";

    // Start db executor actors
    let addr =
        SyncArbiter::start(num_cpus::get() * 3, move || db::DbExecutor::new(db_url));

    // start http server
    Server::build()
        .backlog(1024)
        .bind("techempower", "0.0.0.0:8080", move || {
            HttpService::build().keep_alive(KeepAlive::Os).h1(
                App::new()
                    .data(addr.clone())
                    .service(web::resource("/db").to_async(dbf))
                    .service(web::resource("/test").to(hh))
//                .service(web::resource("/fortune").to_async(fortune))
//                .service(web::resource("/queries").to_async(queries))
//                .service(web::resource("/updates").to_async(updates)))
            )
        })?
        .start();

    println!("Started http server: 127.0.0.1:8080");
    sys.run()
}

use actix_web::{http, Error};
use futures::Future;
use actix_web::dev::HttpResponseBuilder;
//use futures::Future;
//use futures::prelude::*;
fn dbf(db: web::Data<Addr<db::DbExecutor>>)
       -> impl Future<Item=HttpResponse, Error=Error> {
    db.send(db::DocId)
        .from_err()
        .and_then(move |res| match res {
            Ok(row) => {
                Ok(HttpResponse::Ok().body("123"))
            }
            Err(ee) => Ok(HttpResponse::InternalServerError().body(ee.to_string()))
        })
}

fn hh(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().header(
        "cache-control", "private, max - age = ")
        .finish()
}
