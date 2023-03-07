pub mod db;
pub mod items;
pub mod orders;
pub mod stations;
pub mod stationtypes;
pub mod client;
pub mod master;

// create a static var that will be set after runtime
pub static mut STATION_ID: i32 = 0;

use actix_web::{web, App, HttpServer, middleware};

use serde::{Deserialize, Serialize};

use chrono;


#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub items: Vec<i32>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub order_type: String,
    pub stations_completed: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderOutput {
    pub id: i32,
    pub items: Vec<Item>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub time_since_order: i64,
    pub source: String,
    pub order_type: String,
    pub stations_completed: Vec<Station>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub station_type: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Station {
    pub id: i32,
    pub name: String,
    pub avgtime: i32,
    pub station_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StationType {
    pub id: i32,
    pub name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[OpenKDS] Missing server type argument");
        std::process::exit(1);
    }
    let server_type = args[1].split("=").collect::<Vec<&str>>()[1].to_string();

    println!("[OpenKDS] Starting OpenKDS Server");
    println!("[OpenKDS] Copyright (C) 2020 @BenClementt (Elevate Web Services Limited)");
    println!("[OpenKDS] Please see the LICENSE file for more information");

    if server_type == "master" {
        println!("[OpenKDS] Starting OpenKDS Master Server");
        
        /* Start Master Web Server */
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .service(web::scope("").configure(master::master_router))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
        
    } else if server_type == "client" {
        if args.len() < 3 {
            println!("[OpenKDS] Missing station ID");
            std::process::exit(1);
        }
        

        let station_id = args[2].split("=").collect::<Vec<&str>>()[1].to_string();

        unsafe {
            STATION_ID = station_id.parse::<i32>().unwrap();
        }

        let station = stations::get_station(station_id.parse::<i32>().unwrap());

        if station.len() == 0 {
            println!("[OpenKDS] Invalid station ID");
            std::process::exit(1);
        }

        let station_type = stationtypes::get_station_type(station[0].station_type);

        if station_type.len() == 0 {
            println!("[OpenKDS] Invalid station type for station.");
            std::process::exit(1);
        }

        println!("[OpenKDS] Starting OpenKDS Station Server");
        println!("[OpenKDS] Station Name: {}", station[0].name);
        println!("[OpenKDS] Station Type: {}", station_type[0].name);

        /* Start Client Web Server */
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .service(web::scope("").configure(client::api_router))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await

    } else {
        println!("[OpenKDS] Invalid server type");
        std::process::exit(1);
    }
}