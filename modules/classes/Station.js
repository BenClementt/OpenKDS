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
            let itemsOut = [];


            for(let j = 0; j < items.length; j++){
                items[j] = items[j][0];
                if(items[j].station_type == this.station_type){
                    itemsOut.push(items[j]);
                }

            }

            // foeach item in itemsOut, combine the same items into one item with a quantity
            let itemsOut2 = [];
            for(let j = 0; j < itemsOut.length; j++){
                let found = false;
                for(let k = 0; k < itemsOut2.length; k++){
                    if(itemsOut[j].id == itemsOut2[k].id){
                        itemsOut2[k].quantity++;
                        found = true;
                    }
                }
                if(!found){
                    itemsOut[j].quantity = 1;
                    itemsOut2.push(itemsOut[j]);
                }
            }

            itemsOut = itemsOut2;


            let time = Math.floor((Date.now() - orders[i].time) / 1000);
            orders[i].time = time;
            orders[i].items = itemsOut;
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