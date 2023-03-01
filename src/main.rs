pub mod db;

use actix_web::{web, App, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use serde_json::json;

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
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub station_type: i32,
}

#[derive(Serialize, Deserialize)]
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



async fn index() -> impl Responder {
    // dynamically get version
    let version = env!("CARGO_PKG_VERSION");
    web::Json(json!({
        "status": "ok",
        "version": version,
        "message": "OpenKDS API Server",
        
    }))
}

async fn get_orders() -> impl Responder {
    let orders = db::get_orders();
    web::Json(json!({
        "status": "ok",
        "orders": orders,
    }))
}

async fn get_item(item_id: web::Path<i32>) -> impl Responder {
    let items = db::get_item(item_id.into_inner());
    web::Json(json!({
        "status": "ok",
        "item": {
            "id": items[0].id,
            "name": items[0].name,
            "price": items[0].price,
        }
    }))
}

async fn get_station(station_id: web::Path<i32>) -> impl Responder {
    let stations = db::get_station(station_id.into_inner());
    web::Json(json!({
        "status": "ok",
        "station": {
            "id": stations[0].id,
            "name": stations[0].name
        }
    }))
}

async fn get_order_display() -> impl Responder {
    let orders = db::get_orders();
    // make an array of orders with items and other information
    let mut order_display: Vec<serde_json::Value> = Vec::new();
    for order in orders {
        let mut items: Vec<serde_json::Value> = Vec::new();
        for item_id in order.items {
            let item = db::get_item(item_id);
            items.push(json!({
                "id": item[0].id,
                "name": item[0].name,
                "price": item[0].price,
            }));
        }
        order_display.push(json!({
            "id": order.id,
            "items": items,
            "time": order.time,
            "source": order.source,
            "order_type": order.order_type,
            "stations_completed": order.stations_completed,
        }));
    }

    web::Json(json!({
        "status": "ok",
        "order_display": order_display,
    }))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get the args in unix style
    let args: Vec<String> = std::env::args().collect();
    let server_type = args[1].split("=").collect::<Vec<&str>>()[1].to_string();

    println!("[OpenKDS] Starting OpenKDS Server");
    println!("[OpenKDS] Copyright (C) 2020 @BenClementt (Elevate Web Services Limited)");
    println!("[OpenKDS] Please see the LICENSE file for more information");

    if server_type == "master" {
        println!("[OpenKDS] Starting OpenKDS Master Server");
    } else if server_type == "station" {
        // check if args[2] exists
        if args.len() < 3 {
            println!("[OpenKDS] Missing station ID");
            std::process::exit(1);
        }
        let station_id = args[2].split("=").collect::<Vec<&str>>()[1].to_string();
        let station = db::get_station(station_id.parse::<i32>().unwrap());
        if station.len() == 0 {
            println!("[OpenKDS] Invalid station ID");
            std::process::exit(1);
        }
        println!("[OpenKDS] Starting OpenKDS Station Server");
        println!("[OpenKDS] Station Name: {}", station[0].name);
        let station_type = db::get_station_type(station[0].station_type);
        if station_type.len() == 0 {
            println!("[OpenKDS] Invalid station type for station.");
            std::process::exit(1);
        }
        println!("[OpenKDS] Station Type: {}", station_type[0].name);
        let orders = db::get_orders();
        let mut order_count = 0;
        for order in orders {
            let mut items: Vec<serde_json::Value> = Vec::new();
            for item_id in order.items {
                let item = db::get_item(item_id);
                items.push(json!({
                    "id": item[0].id,
                    "name": item[0].name,
                    "price": item[0].price,
                }));
            }
            println!("[OpenKDS] Order #{}: {} - {} - {} - {}", order.id, order.time, order.source, order.order_type, items[0]["name"]);
            order_count += 1;
        }
        println!("[OpenKDS] Current Order Count: {}", order_count);

    } else {
        println!("[OpenKDS] Invalid server type");
        std::process::exit(1);
    }



    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/sys/orders", web::get().to(get_orders))
            .route("/sys/item/{id}", web::get().to(get_item))
            .route("/sys/station/{id}", web::get().to(get_station))

            .route("/api/order_display", web::get().to(get_order_display))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}