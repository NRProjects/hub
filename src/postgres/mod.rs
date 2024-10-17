use tokio_postgres::{Client, Error, NoTls};

pub mod postgres;

pub async fn connect_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect("host=192.168.1.108 port=5432 dbname=storage user=postgres password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}