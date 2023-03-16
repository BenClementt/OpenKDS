import db from "../utils/db.js";

class Item {
    constructor(id, name, price, station_type) {
        this.id = id;
        this.name = name;
        this.price = price;
        this.station_type = station_type;
    }

    async getId() {
        return this.id;
    }

    async getName() {
        return this.name;
    }

    async getPrice() {
        return this.price;
    }

    async getStationTypeId() {
        return this.station_type;
    }

    async getStationTypeName(){
        const [stationTypeData] = await db.query("SELECT * FROM station_types WHERE id = ?", [this.station_type]);
        return stationTypeData[0].name;
    }

    async getStationTypeData(){
        const [stationTypeData] = await db.query("SELECT * FROM station_types WHERE id = ?", [this.station_type]);
        return stationTypeData[0];
    }
}

export default Item;