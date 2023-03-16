import db from "../utils/db.js";
import Order from "./Order.js";

class Station {
    constructor(id, name, avgtime, station_type){
        this.id = id;
        this.name = name;
        this.avgtime = avgtime;
        this.station_type = station_type;
    }

    async getId(){
        return this.id;
    }

    async getName(){
        let type = await this.getStationType();
        return type + "/" + this.name;
    }

    async getStationType(){
        const [stationTypeData] = await db.query("SELECT * FROM station_types WHERE id = ?", [this.station_type]);
        return stationTypeData[0].name;
    }

    async getStationTypeData(){
        const [stationTypeData] = await db.query("SELECT * FROM station_types WHERE id = ?", [this.station_type]);
        return stationTypeData[0];
    }

    async getAllItems(){
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [this.station_type]);
        return items;
    }

    async getAvgTime(){
        return this.avgtime;
    }

    async getAllOrders(){
        const [orders] = await db.query("SELECT * FROM orders WHERE stations_completed != ?", [this.station_type]);

        for(let i = 0; i < orders.length; i++){
            orders[i] = new Order(orders[i].id, orders[i].items, orders[i].time, orders[i].source, orders[i].order_type, orders[i].stations_completed);
            let items = await orders[i].getItemDataInOrder();

            for(let j = 0; j < items.length; j++){
                if(items[j][0].station_type == this.station_type){
                    items[j] = items[j][0];
                }
            }
            let time = Math.floor((Date.now() - orders[i].time) / 1000);
            orders[i].time = time;
            orders[i].items = items;
        }

        return orders;
    }

    async getOrderCount(){
        const [orders] = await db.query("SELECT COUNT(*) FROM orders WHERE stations_completed != ?", [this.station_type]);
        return orders[0]["COUNT(*)"];
    }

    async getPendingCount(){
        const [orders] = await db.query("SELECT COUNT(*) FROM orders WHERE stations_completed != ?", [this.station_type]);
        if(orders[0]["COUNT(*)"] > 4){
            return orders[0]['COUNT(*)'] - 4;
        } else {
            return 0;
        }
    }

}




export default Station;