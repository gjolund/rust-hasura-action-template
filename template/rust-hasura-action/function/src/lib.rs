use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};
    use actix_http::Request;
    use bytes::Bytes;

    fn create_req() -> Request {
      test::TestRequest::with_uri("/").to_request()
    }

    #[actix_rt::test]
    async fn test_handler() {
        let mut app = test::init_service(App::new().service(handler)).await;

        let req = create_req();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = create_req();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(result, Bytes::from_static(b"Hello Actix World!"));
    }
}
