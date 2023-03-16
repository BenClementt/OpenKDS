import db from "../utils/db.js";


class Order{
    constructor(id, items, time, source, order_type, stations_completed){
        this.id = id;
        this.items = items;
        this.time = time;
        this.source = source;
        this.order_type = order_type;
        this.stations_completed = stations_completed;
    }

    async getId(){
        return this.id;
    }

    async getItemIdInOrder(){
        let items = this.items.split(",");
        items = items.filter(function (el) {
            return el != null && el != "";
        });

        return items;
    }

    async getItemDataInOrder(){
        let items = await this.getItemIdInOrder();
        let itemData = [];

        for(let i = 0; i < items.length; i++){
            const item = await db.query("SELECT * FROM items WHERE id = ?", [items[i]]);
            itemData.push(item[0]);
        }

        return itemData;
    }

}

export default Order;