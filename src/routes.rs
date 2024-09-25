use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_health_route() {
        let app = test::init_service(App::new().service(super::health)).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), StatusCode::OK);

        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(body_str, "OK");
    }
}
