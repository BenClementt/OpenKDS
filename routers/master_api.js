import express from "express";
const router = express.Router();

/* Import Custom Modules */
import { checkAuth } from "./master.js";
import db from "../modules/utils/db.js";
import log from "../modules/utils/log.js";

router.get("/stations", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM stations");
    for (let i = 0; i < data.length; i++) {
        const [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[i].station_type]);
        data[i].stationType = stationType[0];
    }

    res.json({
        status: "200",
        error: false,
        data: data
    }) 
});

router.get("/items", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM items");
    for (let i = 0; i < data.length; i++) {
        const [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[i].station_type]);
        data[i].stationType = stationType[0];
    }

    res.json({
        status: "200",
        error: false,
        data: data
    }) 
})

router.get("/stationtypes", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM station_types");
    for (let i = 0; i < data.length; i++) {
        const [stations] = await db.query("SELECT * FROM stations WHERE station_type = ?", [data[i].id]);
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [data[i].id]);
        data[i].stations = stations;
        data[i].items = items;
    }
    
    res.json({
        status: "200",
        error: false,
        data: data
    })
});

router.get("/stationtypes/:id", checkAuth, async (req, res) => {
    const id = req.params.id;
    const [data] = await db.query("SELECT * FROM station_types WHERE id = ?", [id]);

    if(data.length > 0){
        const [stations] = await db.query("SELECT * FROM stations WHERE station_type = ?", [data[0].id]);
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [data[0].id]);
        data[0].stations = stations;
        data[0].items = items;

        res.json({
            status: "200",
            error: false,
            data: data[0]
        })
    } else {
        res.json({
            status: "404",
            error: true,
            data: "Not Found"
        })
    }
});

router.get("/items/:id", checkAuth, async (req, res) => {
    const id = req.params.id;
    const [data] = await db.query("SELECT * FROM items WHERE id = ?", [id]);

    if(data.length > 0){
        const [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[0].station_type]);
        data[0].stationType = stationType[0];
        
        res.json({
            status: "200",
            error: false,
            data: data[0]
        })
    } else {
        res.json({
            status: "404",
            error: true,
            data: "Not Found"
        })
    }
});

router.get("/stations/:id", checkAuth, async (req, res) => {
    const id = req.params.id;
    const [data] = await db.query("SELECT * FROM stations WHERE id = ?", [id]);

    if(data.length > 0){
        const [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[0].station_type]);
        data[0].stationType = stationType[0];
        
        res.json({
            status: "200",
            error: false,
            data: data[0]
        })
    } else {
        res.json({
            status: "404",
            error: true,
            data: "Not Found"
        })
    }
});



export {
    router
};