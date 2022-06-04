use actix_web::{web, App, HttpResponse, HttpServer};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/echo", web::post().to(echo))
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod unittests {
    use super::*;
    use actix_web::{
        body::to_bytes,
        http,
        web::Bytes,
    };

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn test_hello_ok() {
        let resp = hello().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
                   "Hello world!"
        );
    }

    #[actix_web::test]
    async fn test_echo_ok() {
        let resp = echo(String::from("hello")).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "hello"
        );
    }

    #[actix_web::test]
    async fn test_manual_hello_ok() {
        let resp = manual_hello().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body.as_str(),
            "Hey there!"
        );
    }
}
