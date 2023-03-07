/* Import Crates */
use actix_web::{web, Responder, HttpResponse, HttpRequest, Error};
use serde_json::json;

/* Import Modules */
use crate::orders;
use crate::stations;
use crate::stationtypes;

use crate::STATION_ID;

use handlebars::Handlebars;

async fn get_station_orders() -> impl Responder {
    let orders = orders::get_station_orders(unsafe { STATION_ID });
    web::Json(json!({
        "status": "ok",
        "orders": orders,
    }))
}

async fn display_orders() -> impl Responder {
    let orders = orders::get_station_orders(unsafe { STATION_ID });
    let station = stations::get_station(unsafe { STATION_ID });
    let station_type = stationtypes::get_station_type(station[0].station_type);
    let station_name = station_type[0].name.clone() + "/" + &station[0].name;

    let mut handlebars = Handlebars::new();

    let avgtime = station[0].avgtime;

    handlebars
        .register_template_string("index", include_str!("../web/main.hbs"))
        .unwrap();

    if orders.len() > 4 {
        let data = json!({
            "orders": orders,
            "pending": orders.len() - 4,
            "station": {
                "id": unsafe { STATION_ID },
                "name": station[0].name,
                "pretty_name": station_name,
                "type": {
                    "id": station_type[0].id,
                    "name": station_type[0].name,
                },
                "avgtime": avgtime,
            }
        });

        let body = handlebars.render("index", &data).unwrap();

        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(body) 
    }

    let data = json!({
        "orders": orders,
        "pending": 0,
        "station": {
            "id": unsafe { STATION_ID },
            "name": station[0].name,
            "pretty_name": station_name,
            "type": {
                "id": station_type[0].id,
                "name": station_type[0].name,
            },
        }
    });

    let body = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body) 
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
            .service(web::resource("/web").route(web::get().to(display_orders)))
            .service(web::resource("/api/orders").route(web::get().to(get_station_orders)))
            .service(web::resource("/api/ping").route(web::get().to(ping)))

    );
}



