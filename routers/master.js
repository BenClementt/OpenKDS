import express from "express";
import dotenv from "dotenv";
import session from "express-session";
import expressMySqlSession from "express-mysql-session";
import crypto from "crypto";
dotenv.config();

/* Import Modules */
import log from "../modules/utils/log.js";
import db from "../modules/utils/db.js";

/* Import Classes */
import Station from "../modules/classes/Station.js";
import StationType from "../modules/classes/StationType.js";

const router = express.Router();

const sessionStore = new (expressMySqlSession(session))({
    host: process.env.DB_HOST,
    port: process.env.DB_PORT,
    user: process.env.DB_USERNAME,
    password: process.env.DB_PASSWORD,
    database: process.env.DB_NAME,
    waitForConnections: true,
    connectionLimit: 10,
    queueLimit: 0
});





const secret = crypto.randomBytes(64).toString("hex");


router.use(session({
    key: "openkds",
    secret: secret,
    store: sessionStore,
    resave: false,
    saveUninitialized: false,
    cookie: {
        maxAge: 1000 * 60 * 60 * 24 * 7, // 1 week
        httpOnly: false,
        secure: false
    }
}));


async function checkAuth(req, res, next){
    next();
    return;

    if(req.session.authenticated){
        next();
    } else {
        const [data] = await db.query("SELECT * FROM users");
        if(data.length > 0){
            res.redirect("/web/login");
        }
        else {
            res.redirect("/web/setup");
        }
    }
}
        



    


router.get("/", async (req, res) => {
    res.json({
        "status": 200,
        "message": "OK",
        "data": {
            "server_type": "master"
        },
        "error": null,
        "timestamp": Date.now(),
    })
})

router.get("/web", async (req, res) => {
    res.redirect("/web/dashboard");
})

router.get("/web/dashboard", checkAuth, async (req, res) => {
    const [stations] = await db.query("SELECT * FROM stations");
    const [stationTypes] = await db.query("SELECT * FROM station_types");
    const [items] = await db.query("SELECT * FROM items");
    const [orders] = await db.query("SELECT * FROM orders");
    const [users] = await db.query("SELECT * FROM users");

    res.render("master/dashboard.ejs", {
        "stations": stations,
        "stationTypes": stationTypes,
        "items": items,
        "orders": orders,
        "users": users
    })
})

router.get("/web/stations", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM stations");
    
    for (let i = 0; i < data.length; i++) {
        data[i].avgtime = `${data[i].avgtime} seconds`
        let [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[i].station_type]);
        if(stationType.length > 0){
            data[i].station_type = stationType[0].name;
        } else {
            data[i].station_type = "Unknown (ID: " + data[i].station_type + ")";
        }
    }

    res.render("master/stations.ejs", {
        "stations": data
    })
})

router.get("/web/stations/:id/edit", checkAuth, async (req, res) => {
    const id = req.params.id;

    const [data] = await db.query("SELECT * FROM stations WHERE id = ?", [id]);

    if(data.length > 0){
        const [stationTypes] = await db.query("SELECT * FROM station_types", [data[0].station_type]);
        const stationType = stationTypes.find((stationType) => stationType.id == data[0].station_type);
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [data[0].station_type]);
        res.render("master/edit/station.ejs", {
            "station": data[0],
            "stationtype": stationType,
            "stationtypes": stationTypes,
            "items": items
        })
    } else {
        res.redirect("/web/stations");
    }
})

router.get("/web/stationtypes", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM station_types");
    for (let i = 0; i < data.length; i++) {
        const [stations] = await db.query("SELECT * FROM stations WHERE station_type = ?", [data[i].id]);
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [data[i].id]);
        data[i].stations = stations;
        data[i].items = items;
    }

    res.render("master/stationtypes.ejs", {
        "stationtypes": data
    })
})

router.get("/web/stationtypes/:id/edit", checkAuth, async (req, res) => {
    const id = req.params.id;
    const [data] = await db.query("SELECT * FROM station_types WHERE id = ?", [id]);

    if(data.length > 0){
        const [stations] = await db.query("SELECT * FROM stations WHERE station_type = ?", [data[0].id]);
        const [items] = await db.query("SELECT * FROM items WHERE station_type = ?", [data[0].id]);
        data[0].stations = stations;
        data[0].items = items;

        res.render("master/edit/stationtype.ejs", {
            "stationtype": data[0]
        })
    } else {
        res.redirect("/web/stationtypes");
    }
})

router.get("/web/items", checkAuth, async (req, res) => {
    const [data] = await db.query("SELECT * FROM items");
    for (let i = 0; i < data.length; i++) {
        const [stationType] = await db.query("SELECT * FROM station_types WHERE id = ?", [data[i].station_type]);
        data[i].stationType = stationType[0];
    }

    res.render("master/items.ejs", {
        "items": data
    });
});

router.get("/web/items/:id/edit", checkAuth, async (req, res) => {
    const id = req.params.id;
    const [data] = await db.query("SELECT * FROM items WHERE id = ?", [id]);

    if(data.length > 0){
        const [stationTypes] = await db.query("SELECT * FROM station_types");
        const stationType = stationTypes.find((stationType) => stationType.id == data[0].station_type);
        res.render("master/edit/item.ejs", {
            "item": data[0],
            "stationtype": stationType,
            "stationtypes": stationTypes
        })
    } else {
        res.redirect("/web/items");
    }
})




router.get("/web/login", async (req, res) => {
    if(req.session.authenticated){
        res.redirect("/web/dashboard");
    } else {
        res.render("master/login.ejs")
    }
})

router.post("/web/login", async (req, res) => {
    const username = req.body.username;
    const password = req.body.password;

    const [data] = await db.query("SELECT * FROM users WHERE username = ?", [username]);

    if(data.length > 0){
        if(data[0].password == password){
            req.session.authenticated = true;
            req.session.username = username;
            res.json({
                "status": 200,
                "message": "OK",
                "data": null,
                "error": null,
                "timestamp": Date.now()
            })
        } else {
            res.json({
                "status": 400,
                "message": "Bad Request",
                "data": null,
                "error": "Invalid password",
                "timestamp": Date.now()
            })
        }
    } else {
        res.json({
            "status": 400,
            "message": "Bad Request",
            "data": null,
            "error": "Invalid username",
            "timestamp": Date.now()
        })
    }
});

router.get("/web/logout", async (req, res) => {
    req.session.destroy();
    res.redirect("/web/login");
})

router.get("/web/setup", async (req, res) => {
    res.render("master/setup.ejs")
})

router.post("/web/setup", async (req, res) => {
    const username = req.body.username;
    const password = req.body.password;

    const [data] = await db.query("SELECT * FROM users");

    if(data.length > 0){
        res.json({
            "status": 400,
            "message": "Bad Request",
            "data": null,
            "error": "User already exists",
            "timestamp": Date.now()
        })
    } else {
        await db.query("INSERT INTO users (username, password) VALUES (?, ?)", [username, password]);
        res.json({
            "status": 200,
            "message": "OK",
            "data": null,
            "error": null,
            "timestamp": Date.now()
        })
    }
});







export default router;


