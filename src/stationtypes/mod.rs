use crate::StationType;
use crate::db::get_pool;

use mysql::*;
use mysql::prelude::*;

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

pub fn get_station_types() -> Vec<StationType> {
    let mut station_types: Vec<StationType> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM station_types").unwrap();
    let result = conn.exec_iter(stmt, ()).unwrap();
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

pub fn add_station_type(name: String) -> Vec<StationType> {
    let mut station_types: Vec<StationType> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO station_types (name) VALUES (:name)").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "name" => name,
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