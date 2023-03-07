use crate::Item;
use crate::Order;

use crate::OrderOutput;
use crate::Station;
use crate::db::get_pool;

use crate::stations;
use crate::items;

use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;


pub fn get_orders() -> Vec<Order> {
    let mut orders: Vec<Order> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    conn.query_iter("SELECT * FROM orders").unwrap()
        .for_each(|row| {
            let (id, items, time, source, order_type, stations_completed): (i32, String, i64, String, String, String) = from_row(row.unwrap());
            let items: Vec<i32> = items.split(",").map(|s| s.parse().unwrap_or(0)).collect();
            let time = Utc.timestamp_opt(time, 0).single().unwrap();
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

pub fn get_order(order_id: i32) -> Vec<Order> {
    let mut orders: Vec<Order> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM items WHERE id = :id").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "id" => order_id,
    }).unwrap();
    result.for_each(|row| {
        let (id, items, time, source, order_type, stations_completed): (i32, String, i64, String, String, String) = from_row(row.unwrap());
        let items: Vec<i32> = items.split(",").map(|s| s.parse().unwrap_or(0)).collect();
        let time = Utc.timestamp_opt(time, 0).single().unwrap();
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

pub fn add_order(order: Order) -> bool {
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let items: String = order.items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    let stations_completed: String = order.stations_completed.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    let stmt = conn.prep("INSERT INTO orders (items, time, source, order_type, stations_completed) VALUES (:items, :time, :source, :order_type, :stations_completed)").unwrap();
    let _result = conn.exec_drop(stmt, params! {
        "items" => items,
        "time" => order.time.timestamp(),
        "source" => order.source,
        "order_type" => order.order_type,
        "stations_completed" => stations_completed,
    }).unwrap();
    return true;
}

// change top line as we are outputting an array of orders


pub fn get_station_orders(station_id: i32) -> Vec<OrderOutput> {
    let mut orders: Vec<OrderOutput> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM orders ORDER BY time ASC;").unwrap();
    let result = conn.exec_iter(stmt, ()).unwrap();

    let station_type = stations::get_station(station_id)[0].station_type;

    result.for_each(|row| {
        let (id, items, time, source, order_type, stations_completed): (i32, String, i64, String, String, String) = from_row(row.unwrap());
        let items: Vec<i32> = items.split(",").map(|s| s.parse().unwrap_or(0)).collect();
        let time = Utc.timestamp_opt(time, 0).single().unwrap();
        let stations_completed: Vec<i32> = stations_completed.split(",").map(|s| s.parse().unwrap_or(0)).collect();
        if stations_completed.contains(&station_id) {
            return;
        }
        let order = Order {
            id,
            items,
            time,
            source,
            order_type,
            stations_completed,
        };
        let mut order_items: Vec<i32> = Vec::new();
        for item in order.items {
            let item = &items::get_item(item)[0];
            if item.station_type == station_type {
                order_items.push(item.id);
            }
        }
        if order_items.len() > 0 {
            let time_since = Utc::now().signed_duration_since(time);
            // put the items in an vec of items
            let mut order_items_out: Vec<Item> = Vec::new();
            for item in order_items {
                let item = items::get_item(item)[0].clone();
                order_items_out.push(item);
            }

            let mut stations_completed: Vec<Station> = Vec::new();
            for station_id in order.stations_completed {
                let station = stations::get_station(station_id)[0].clone();
                stations_completed.push(station);
            }


            let order = OrderOutput {
                id,
                items: order_items_out,
                time,
                source: order.source,
                time_since_order: time_since.num_seconds(),
                order_type: order.order_type,
                stations_completed,
            };
            orders.push(order);
        }
    });

    // output
    return orders;


    
}
