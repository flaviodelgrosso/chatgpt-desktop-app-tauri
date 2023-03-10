import { execSync } from "child_process";
import fs from "fs";
import k from "kleur";

import { $argv, tauriConfJSON, relativePath, TAURI_CONF_PATH, PACKAGE_PATH, packageJSON } from "./utils.js";

/** Args can be: --git --minor --major */
const argv = $argv();
const tauriConfJson = tauriConfJSON();
const pkg = packageJSON();
const version = tauriConfJson?.package?.version || "0.0.0";
let [a, b, c] = version.split(".")?.map(Number);

if (argv.major) {
  a += 1;
  b = 0;
  c = 0;
} else if (argv.minor) {
  b += 1;
  c = 0;
} else {
  c += 1;
}

const nextVersion = `${a}.${b}.${c}`;
const oldVersion = version;
tauriConfJson.package.version = nextVersion;
pkg.version = nextVersion;

// rewrite tauri.conf.json
fs.writeFileSync(TAURI_CONF_PATH, JSON.stringify(tauriConfJson, null, 2));
// rewrite package.json
fs.writeFileSync(PACKAGE_PATH, JSON.stringify(pkg, null, 2));
const verChange = `(${k.yellow(oldVersion)} -> ${k.green(nextVersion)})`;

if (argv.git) {
  execSync(`git add ${relativePath(TAURI_CONF_PATH)} ${relativePath(PACKAGE_PATH)}`);
  execSync(`git commit -m "v${nextVersion}"`);
  execSync(`git tag -a v${nextVersion} -m "v${nextVersion}"`);
  execSync(`git push`);
  execSync(`git push origin v${nextVersion}`);
  console.log(k.green("[✨ release]"), `Published successfully ${verChange}`);
} else {
  console.log(k.green("[✨ release]"), `tauri.conf.json version ${verChange}`);
  console.log(k.green("[✨ release]"), `package.json version ${verChange}`);
}
