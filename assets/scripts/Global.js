module.exports = {
    project: {
        version: "1.0.0",
        homePage: "https://github.com/cuso4-5h2o/protamine-meowgame"
    },
    settings: cc.sys.localStorage.getItem("settings") ? JSON.parse(cc.sys.localStorage.getItem("settings")) : {
        speed: 1,
        volume: 0.5,
        enable: {
            dayNight: true,
            particle: true,
            rotation: true,
            getFlag: false
        }
    },
    ID: "",
    fragments: cc.sys.localStorage.getItem("fragments") ? parseInt(cc.sys.localStorage.getItem("fragments")) : 0,
    highestScore: cc.sys.localStorage.getItem("highestScore") ? parseInt(cc.sys.localStorage.getItem("highestScore")) : 0,
    magicStars: {
        blue: 1,
        red: 1,
        yellow: 1,
        green: 1
    },
    score: 0,
    pickingLog: [],
    ballPosition: null,
    isPlaying: false,
    isClimbing: false,
    pickingRadius: 32,
    speedPlaying: 1,
    speedClimbing: 1,
    scoreTimes: 1,
    haloNode: null,
    particleNode: null,
    magicStarStatuses: {},
    invincibleTime: false,
    saveData() {
        cc.sys.localStorage.setItem("fragments", this.fragments.toString());
        cc.sys.localStorage.setItem("highestScore", this.highestScore.toString());
        cc.sys.localStorage.setItem("settings", JSON.stringify(this.settings));
        cc.sys.localStorage.setItem("magicStars", JSON.stringify(this.magicStars));
    }
};
