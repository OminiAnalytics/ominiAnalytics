const path = require("path");

module.exports = {
    entry: "./src/ominiTracker.js",
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
