const path = require("path");

module.exports = {
    entry: "./src/index.js",
    target: "web",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
        library: "OminiTracker",
        libraryTarget: "umd",
        globalObject: "this",
        libraryExport: "default",
        umdNamedDefine: true,
    },
};
