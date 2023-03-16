import express from "express";
import dotenv from "dotenv";
dotenv.config();

/* Import Classes */
import Station from "../modules/classes/Station.js";
import StationType from "../modules/classes/StationType.js";

const router = express.Router();

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

export default router;


