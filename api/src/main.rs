use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sol::SolClient;

const PORT:&'static str = "8080";

#[get("/")]
async fn hello_world() -> impl Responder{
    "hello_world"
}


#[get("/balance/{address}")]
async fn get_balance(path:web::Path<String>) -> impl Responder {
    let address = path.into_inner();
    let sol_client = SolClient::new();
    let res = sol_client.get_balance(&address).await;
    HttpResponse::Ok().json(res)
}

#[get("/token/create")]
async fn create_token() -> impl Responder{
    let sol_client = SolClient::new();
    let res = sol_client.create_token().await;
    HttpResponse::Ok().json(res)
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error>{

    let address = format!("0.0.0.0:{}", PORT);

    println!("running server at port : {}", PORT);

    HttpServer::new(||{
        App::new()
        .service(
            web::scope("/api")
            .service(
                hello_world
            )
            .service(
                get_balance
            )
            .service(
                create_token
            )
        )           
    })
    .bind(address)
    .expect("Failed to bind to the port")
    .run()
    .await
}
