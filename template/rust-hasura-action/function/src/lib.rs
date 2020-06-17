use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
