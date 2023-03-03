/* Import Crates */
use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono;

/* Import Modules */
use crate::db;
use crate::items;
use crate::orders;
use crate::stations;
use crate::stationtypes;
use crate::client;

/* Import Structs */
use crate::Order;
use crate::Item;
use crate::Station;
use crate::StationType;

use crate::STATION_ID;

async fn get_station_orders() -> impl Responder {
    let orders = orders::get_station_orders(unsafe { STATION_ID });
    web::Json(json!({
        "status": "ok",
        "orders": orders,
    }))
}

async fn ping() -> impl Responder {
    let station = stations::get_station(unsafe { STATION_ID });
    let station_type = stationtypes::get_station_type(station[0].station_type);
    let station_name = station[0].name.clone() + "/" + &station_type[0].name;


    web::Json(json!({
        "status": "ok",
        "message": "pong",
        "station": {
            "id": unsafe { STATION_ID },
            "name": station[0].name,
            "pretty_name": station_name,
            "type": {
                "id": station_type[0].id,
                "name": station_type[0].name,
            },
        }
    }))
}


async fn index() -> impl Responder {
    let version = env!("CARGO_PKG_VERSION");
    web::Json(json!({
        "status": "ok",
        "version": version,
        "message": "OpenKDS - Client API Server",
        "station_id": unsafe { STATION_ID },
    }))
}

// create seperate router
pub(crate) fn api_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/orders").route(web::get().to(get_station_orders)))
            .service(web::resource("/ping").route(web::get().to(ping)))

    );
}



