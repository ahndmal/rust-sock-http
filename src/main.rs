use std::io::{Read, Write};
use std::{net, time};
use std::alloc::System;
use std::fmt::Error;
use std::future::Future;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

// use mongodb::{bson::doc, Client};
use mongodb::{bson::doc, Client};
use mongodb::options::ClientOptions;

// use mongodb::options::ClientOptions;

// fn mongo_sync() -> mongodb::error::Result<()> {
//     // let client = Client::with_uri_str(
//     //     std::env::var("DB_URL").unwrap(),
//     // );
//
//     let db_url = std::env::var("DB_URL").unwrap();
//     let client = Client::with_options(
//         ClientOptions::parse(db_url)
//     );
//
//     client
//         .database("workouts")
//         .run_command(doc! {"ping": 1}, None)?;
//
//     println!("Connected successfully.");
//
//     for db_name in client.list_database_names(None, None)? {
//         println!("{}", db_name);
//     }
//     Ok(())
// }

// async fn mongo_conn() -> &'static str {
//     let mut client_options =
//         ClientOptions::parse(std::env::var("DB_URL").unwrap()).await.unwrap();
//
//     client_options.app_name = Some("Http workouts".to_string());
//
//     let client = Client::with_options(client_options).unwrap();
//
//     client
//         .database("workouts")
//         .run_command(doc! {"ping": 1}, None)
//         .await.unwrap();
//     println!("Connected successfully.");
//     for db_name in client.list_database_names(None, None).await.unwrap() {
//         println!("{}", db_name);
//     }
//
//     return "data";
// }

#[tokio::main]
async fn main() {
    // mongo_sync().unwrap();
    let port = 8083;
    let server = net::TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("[ TCP ] Server started on {} with port {}", server.local_addr().unwrap().ip(), port);

    loop {
        let (mut tcp_stream, sock_addr) = server.accept().unwrap();
        println!("[ TCP ] new tcp_stream client: {tcp_stream:?}");

        let ip_addr = sock_addr.ip();

        println!("{}", ip_addr);

        // let system_time = time::SystemTime::now();
        let mut data = "HTTP/1.1 200 OK\nContent-Type: text/html\r\n\n<h2>Hello from Rust</h2>"
            .as_bytes();

        let addr = tcp_stream.local_addr().unwrap();
        println!("[ TCP ] Address: {:?}", addr);

        tcp_stream.write_all(data).expect("Could not write to client socket 😎 ");

    }
}

// fn sock_async_custom() {
//     while true {
//         match server.accept() {
//             Ok((mut sock, addr)) => {
//                 // time::SystemTime::now()
//                 let mut data = "HTTP/1.1 200 OK\nContent-Type: text/html\nServer: Rust\n
//                 <h2>Hello from Rust</h2>".as_bytes();
//                 println!("new client: {addr:?}");
//
//                 // let mut resp_str = "".to_string();
//                 // sock.read_to_string(&mut resp_str).unwrap();
//                 // println!("{}", resp_str);
//
//                 match sock.local_addr() {
//                     Ok(sock_addr) => {
//                         println!("{:?}", sock_addr);
//                     }
//                     Err(err) => {}
//                 }
//                 sock.write_all(data).expect("Could not write to client socket 😎 ");
//             }
//             Err(e) => println!("couldn't get client: {e:?}"),
//         }
//     }
// }
