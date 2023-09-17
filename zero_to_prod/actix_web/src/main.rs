use zero_to_prod_actix_web::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}