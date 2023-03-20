/* Module Imports */
import express from "express";
import dotenv from "dotenv";
dotenv.config();

/* Import Classes */
import Station from "./modules/classes/Station.js";
import StationType from "./modules/classes/StationType.js";

/* Initialise Variables */
const port = process.env.PORT || 3000;
const server_type = process.argv.slice(2)[0] || "master";
const station_id = process.argv.slice(2)[1] || -1;

/* Initialise Express */
const app = express();
app.use(express.urlencoded({ extended: true }));
app.use(express.json());


import masterRouter from "./routers/master.js";



/* Import Custom Modules */
import log from "./modules/utils/log.js";
import db from "./modules/utils/db.js";





/* Run Express Server */
app.listen(port, async () => {
    try {
        await db.query("SELECT 1");
        log("Database connection established.");
    } catch (error) {
        log("Database connection failed.");
        log(error);
    }

    if(server_type == "master") {
        app.use("/", masterRouter);
        log("Master server started.");

    } else if (server_type == "client") {
        if(station_id == -1) {
            log("No station ID provided. Exiting.");
            process.exit(1);
        }
        const [stationData] = await db.query("SELECT * FROM stations WHERE id = ?", [station_id]);

        if(stationData.length == 0) {
            log("Invalid station ID provided. Exiting.");
            process.exit(1);
        }

        const station = new Station(stationData[0].id, stationData[0].name, stationData[0].avgtime, stationData[0].station_type);

        log(`Station ID: ${station_id}`);
        log(`Station Name: ${await station.getName()}`);
        app.use("/", await import("./routers/client.js").then((module) => module.default));
    } else {
        log("Invalid server type provided. Exiting.");
        process.exit(1);
    }


    log(`Server is running on port ${port}.`)
    
});

export {
    station_id
}