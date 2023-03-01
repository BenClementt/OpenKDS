use chrono::prelude::*;

use dotenv::dotenv;
use std::env;

use crate::{Order, Item, Station, StationType};

use mysql::*;
use mysql::prelude::*;



pub fn get_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_opts = Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(connection_opts).unwrap();
    return pool;
}

pub fn get_orders() -> Vec<Order> {
    let mut orders: Vec<Order> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    conn.query_iter("SELECT * FROM orders").unwrap()
        .for_each(|row| {
            let (id, items, time, source, order_type, stations_completed): (i32, String, i64, String, String, String) = from_row(row.unwrap());
            let items: Vec<i32> = items.split(",").map(|s| s.parse().unwrap_or(0)).collect();
            let time = Utc.timestamp(time, 0);
            let stations_completed: Vec<i32> = stations_completed.split(",").map(|s| s.parse().unwrap_or(0)).collect();
            let order = Order {
                id,
                items,
                time,
                source,
                order_type,
                stations_completed,
            };
            orders.push(order);
        });

    return orders;
}

pub fn get_item(item_id: i32) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM items WHERE id = :id").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "id" => item_id,
    }).unwrap();
    result.for_each(|row| {
        let (id, name, price, station_type): (i32, String, f32, i32) = from_row(row.unwrap());
        let item = Item {
            id,
            name,
            price,
            station_type,
        };
        items.push(item);
    });


    return items;
    
}

pub fn get_station(station_id: i32) -> Vec<Station> {
    let mut stations: Vec<Station> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM stations WHERE id = :id").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "id" => station_id,
    }).unwrap();
    result.for_each(|row| {
        let (id, name, avgtime, station_type): (i32, String, i32, i32) = from_row(row.unwrap());
        let station = Station {
            id,
            name,
            avgtime,
            station_type,
        };
        stations.push(station);
    });
    return stations;
}

pub fn get_station_type(station_type_id: i32) -> Vec<StationType> {
    let mut station_types: Vec<StationType> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM station_types WHERE id = :id").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "id" => station_type_id,
    }).unwrap();
    result.for_each(|row| {
        let (id, name): (i32, String) = from_row(row.unwrap());
        let station_type = StationType {
            id,
            name,
        };
        station_types.push(station_type);
    });
    return station_types;
}




