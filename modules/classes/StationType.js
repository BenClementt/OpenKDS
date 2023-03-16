class StationType {
    constructor(name, id) {
        this.name = name;
        this.id = id;

    }

    async getId(){
        return this.id;
    }

    async getName(){
        return this.name;
    }

    async getStations() {
        const [stations] = await db.query("SELECT * FROM stations WHERE station_type = ?", [this.id]);
        return stations;
    }

    async getStationCount() {
        const [stations] = await db.query("SELECT COUNT(*) FROM stations WHERE station_type = ?", [this.id]);
        return stations[0]["COUNT(*)"];
    }

    async getStationTypeAverageTime() {
        const [stations] = await db.query("SELECT AVG(avgtime) FROM stations WHERE station_type = ?", [this.id]);
        return stations[0]["AVG(avgtime)"];
    }

    async getStationTypeAverageTimeFormatted() {
        const [stations] = await db.query("SELECT AVG(avgtime) FROM stations WHERE station_type = ?", [this.id]);
        return new Date(stations[0]["AVG(avgtime)"] * 1000).toISOString().substr(11, 8);
    }

}

export default StationType;