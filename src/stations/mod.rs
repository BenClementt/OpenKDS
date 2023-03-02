use crate::Station;
use crate::db::get_pool;

use mysql::*;
use mysql::prelude::*;

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

pub fn get_stations() -> Vec<Station> {
    let mut stations: Vec<Station> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM stations").unwrap();
    let result = conn.exec_iter(stmt, ()).unwrap();
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

pub fn get_stations_by_type(station_type: i32) -> Vec<Station> {
    let mut stations: Vec<Station> = Vec::new();
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("SELECT * FROM stations WHERE station_type = :station_type").unwrap();
    let result = conn.exec_iter(stmt, params! {
        "station_type" => station_type,
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

pub fn add_station(name: String, station_type: i32) -> bool {
    let pool = get_pool();
    let mut conn = pool.get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO stations (name, avgtime, station_type) VALUES (:name, :avgtime, :station_type)").unwrap();
    let _result = conn.exec_drop(stmt, params! {
        "name" => name,
        "avgtime" => 0,
        "station_type" => station_type,
    }).unwrap();
    return true;
}