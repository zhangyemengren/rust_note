use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use zero_to_prod_actix_web::configuration::get_configuration;
use zero_to_prod_actix_web::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(
        &configuration.database.connection_string()
    ).await.expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}