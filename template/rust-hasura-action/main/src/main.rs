use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(function::handler)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
