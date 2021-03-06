var Global = require("Global");
var Costs = [10, 20, 40, 80, 160];

cc.Class({
    extends: cc.Component,
    properties: {
        magicStarIconNode: cc.Node,
        magicStarDescriptionNode: cc.Node,
        magicStarNameLabel: cc.Label,
        magicStarDescriptionLabel: cc.Label,
        toastPrefab: cc.Prefab
    },
    init(data) {
        this.magicStarId = data.id;
        this.magicStarNameLabel.string = data.name;
        this.magicStarIconNode.color = data.color;
        this.magicStarDescriptionLabel.string = data.description;
        this.magicStarDescriptionNode.opacity = 255;
    }
});
