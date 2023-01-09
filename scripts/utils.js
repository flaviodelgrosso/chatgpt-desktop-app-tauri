import path from "path";
import minimist from "minimist";

export const $argv = () => minimist(process.argv.slice(2));
export const relativePath = (p) => `${p.split(ROOT_PATH)?.[1].substring(1)}` || "";

export const ROOT_PATH = process.cwd();
export const TAURI_CONF_PATH = path.join(ROOT_PATH, "src-tauri", "tauri.conf.json");
export const UPDATER_JSON_PATH = path.join(ROOT_PATH, "updater", "releases.json");
