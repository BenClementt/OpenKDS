import express from "express";
import dotenv from "dotenv";
dotenv.config();

/* Import Modules */
import log from "../modules/utils/log.js";
import db from "../modules/utils/db.js";

/* Import Classes */
import Station from "../modules/classes/Station.js";
import StationType from "../modules/classes/StationType.js";
import Item from "../modules/classes/Item.js";
import Order from "../modules/classes/Order.js";

/* Initialise Variables */
import { station_id } from "../index.js";

const [stationData] = await db.query("SELECT * FROM stations WHERE id = ?", [station_id]);
const station = new Station(stationData[0].id, stationData[0].name, stationData[0].avgtime, stationData[0].station_type);

const router = express.Router();

router.get("/", async (req, res) => {
    res.json({
        "status": 200,
        "message": "OK",
        "data": {
            "server_type": "client",
            "station_id": await station.getId(),
            "station_name": await station.getName(),
        },
        "timestamp": Date.now() / 1000,
    })
});

router.get("/orders/:type", async (req, res) => {
    if(req.params.type == "json"){
        const orders = await station.getAllOrders();
        const pending = await station.getPendingCount();
        log("Request to /orders. Orders: " + orders.length);
        res.json({
            "status": 200,
            "message": "OK",
            "data": {
                "orders": orders,
                "pending": {
                    "status": pending > 0 ? "true" : "false",
                    "count": pending
                }
            },
            "timestamp": Date.now() / 1000,
        });
    } else if(req.params.type == "html"){
        const orders = await station.getAllOrders();
        const pending = await station.getPendingCount();
        res.render("partials/orders.ejs", {
            "orders": orders,
            "pending":{
                "status": pending > 0 ? true : false,
                "count": pending
            },
            "station":{
                "id": await station.getId(),
                "name": await station.getName(),
                "avgtime": await station.getAvgTime(),
                "type": await station.getStationTypeData()
            }
        });
    }
});

router.get("/web", async (req, res) => {
    const orders = await station.getAllOrders();
    const pending = await station.getPendingCount();
    res.render("client/index.ejs", {
        "station":{
            "id": await station.getId(),
            "name": await station.getName(),
            "avgtime": await station.getAvgTime(),
            "type": await station.getStationTypeData()
        }
    });
})





    







export default router;


