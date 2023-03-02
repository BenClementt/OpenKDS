use crate::Order;
use crate::db::get_pool;

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