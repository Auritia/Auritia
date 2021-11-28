const fs = require("fs");

const { version } = require("../package.json");

const tauriConfig = require("../src-tauri/tauri.conf.json");
const tauriLinuxConfig = require("../src-tauri/tauri.linux.conf.json");

tauriConfig.package.version = version;
tauriLinuxConfig.package.version = version;

fs.writeFileSync("./src-tauri/tauri.conf.json", JSON.stringify(tauriConfig, null, 2));
fs.writeFileSync("./src-tauri/tauri.linux.conf.json", JSON.stringify(tauriLinuxConfig, null, 2));

process.exit(0);
