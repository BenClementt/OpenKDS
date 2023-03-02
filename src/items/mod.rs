use crate::Item;

use mysql::*;
use mysql::prelude::*;
use crate::db::get_pool;

pub fn get_items() -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    conn.query_iter("SELECT * FROM items").unwrap()
        .for_each(|row| {
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

pub fn add_item(item: Item) -> bool {
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO items (name, price, station_type) VALUES (:name, :price, :station_type)").unwrap();
    let _result = conn.exec_drop(stmt, params! {
        "name" => item.name,
        "price" => item.price,
        "station_type" => item.station_type,
    }).unwrap();
    return true;
}