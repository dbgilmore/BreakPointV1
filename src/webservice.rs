use actix_web::{post, App, HttpResponse, HttpServer, Responder};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[actix_web::test]
async fn test_echo_with_reqwest() {
    // send a request via reqwest to 127.0.0.1:8080/echo
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/echo")
        .body("Hello, world!")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(res, "Hello, world!");
}

