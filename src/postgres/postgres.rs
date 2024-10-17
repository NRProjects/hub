// pub async fn connect_db() -> Result<Client, Error> {
//     let (client, connection) = tokio_postgres::connect("host=localhost dbname=storage user=postgres", NoTls).await?;

//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });

//     let rows = client.query("SELECT columnname FROM TEST", &[]).await?;

//     for row in rows {
//         let value: &str = row.get(0);
//         println!("value: {}", value);
//     }

//     Ok(())

//     Ok(())
// }