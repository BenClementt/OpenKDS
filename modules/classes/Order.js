import db from "../utils/db.js";


class Order{
    constructor(id){
        this.id = id;
    }

    getId(){
        return this.id;
    }
    
    async getItems(){
        const [items] = await db.query("SELECT items FROM orders WHERE id = ?", [this.id]);
        return items[0].items;
    }

    async getUnixTime(){
        const [time] = await db.query("SELECT time FROM orders WHERE id = ?", [this.id]);
        return time[0].time;
    }

    async getUnixTimeSince(){
        const [time] = await db.query("SELECT time FROM orders WHERE id = ?", [this.id]);
        return Date.now() / 1000 - time[0].unixtime;
    }

    async getSource(){
        const [source] = await db.query("SELECT source FROM orders WHERE id = ?", [this.id]);
        return source[0].source;
    }

    async getOrderType(){
        const [order_type] = await db.query("SELECT order_type FROM orders WHERE id = ?", [this.id]);
        return order_type[0].order_type;
    }

    async getCompletedStations(){
        const [stations_completed] = await db.query("SELECT stations_completed FROM orders WHERE id = ?", [this.id]);
        return stations_completed[0].stations_completed;
    }

    async serve(station_id){
        const [stations_completed] = await db.query("SELECT stations_completed FROM orders WHERE id = ?", [this.id]);
        const stations = stations_completed[0].stations_completed.split(",");
        stations.push(station_id);
        await db.query("UPDATE orders SET stations_completed = ? WHERE id = ?", [stations.join(","), this.id]);

        return true;
    }



    async getItemIds(){
        let items = await this.getItems();
        items = items.split(",");
        items = items.filter(function (el) {
            return el != null && el != "";
        });

        return items;
    }

    async getItemDataInOrder(){
        let items = await this.getItemIds();
        let itemData = [];

        for(let i = 0; i < items.length; i++){
            const item = await db.query("SELECT * FROM items WHERE id = ?", [items[i]]);
            itemData.push(item[0]);
        }

        return itemData;
    }

}

export default Order;