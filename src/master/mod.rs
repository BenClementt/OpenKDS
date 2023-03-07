use actix_web::{web, Responder};

use serde_json::json;

/* Impport Modules */
use crate::items;
use crate::orders;
use crate::stations;

async fn index() -> impl Responder {
    let version = env!("CARGO_PKG_VERSION");
    web::Json(json!({
        "status": "ok",
        "version": version,
        "message": "OpenKDS - Master API Server",
        
    }))
}

async fn get_orders() -> impl Responder {
    let orders = orders::get_orders();
    web::Json(json!({
        "status": "ok",
        "orders": orders,
    }))
}

async fn get_item(item_id: web::Path<i32>) -> impl Responder {
    let items = items::get_item(item_id.into_inner());
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
    let stations = stations::get_station(station_id.into_inner());
    web::Json(json!({
        "status": "ok",
        "station": {
            "id": stations[0].id,
            "name": stations[0].name
        }
    }))
}

async fn get_order_display() -> impl Responder {
    let orders = orders::get_orders();
    // make an array of orders with items and other information
    let mut order_display: Vec<serde_json::Value> = Vec::new();
    for order in orders {
        let mut items: Vec<serde_json::Value> = Vec::new();
        for item_id in order.items {
            let item = items::get_item(item_id);
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


pub(crate) fn master_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/api/orders").route(web::get().to(get_orders)))
            .service(web::resource("/api/orders/display").route(web::get().to(get_order_display)))
            .service(web::resource("/api/item/{item_id}").route(web::get().to(get_item)))
            .service(web::resource("/api/station/{station_id}").route(web::get().to(get_station)))
    );
}



