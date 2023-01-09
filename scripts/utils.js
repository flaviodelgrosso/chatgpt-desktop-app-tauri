import path from "path";
import minimist from "minimist";
import { createRequire } from "module";

const require = createRequire(import.meta.url);

export const ROOT_PATH = process.cwd();
export const PACKAGE_PATH = path.join(ROOT_PATH, "package.json");
export const TAURI_CONF_PATH = path.join(ROOT_PATH, "src-tauri", "tauri.conf.json");
export const UPDATER_JSON_PATH = path.join(ROOT_PATH, "updater", "releases.json");

export const $argv = () => minimist(process.argv.slice(2));
export const relativePath = (p) => `${p.split(ROOT_PATH)?.[1].substring(1)}` || "";
export const tauriConfJSON = () => require(TAURI_CONF_PATH);
export const packageJSON = () => require(PACKAGE_PATH);
