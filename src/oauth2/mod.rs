use actix_web::{get, HttpRequest, HttpResponse, post, Responder};

#[get("/auth")]
pub async fn auth_redirect(req: HttpRequest) -> impl Responder {
    const ACCOUNT_SERVER: &str = "https://accounts.stella-it.com/auth";
    let query_string: String = req.query_string().to_string();

    HttpResponse::TemporaryRedirect().header("Location", format!("{}?{}", ACCOUNT_SERVER, query_string)).body("")
}

#[get("/token")]
pub async fn get_token() -> impl Responder {
    HttpResponse::Ok().body("")
}

async fn is_client_valid(client_id: String, client_secret: String) {

}