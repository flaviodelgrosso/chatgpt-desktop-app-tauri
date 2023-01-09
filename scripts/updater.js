import fs from "fs";
import path from "path";
import fetch from "node-fetch";
import { getOctokit, context } from "@actions/github";
import c from "kleur";
import minimist from "minimist";

const $argv = () => minimist(process.argv.slice(2));

(async () => {
  const argv = $argv();
  const ROOT_PATH = process.cwd();
  const UPDATER_JSON_PATH = path.join(ROOT_PATH, "updater", "releases.json");
  const relativePath = (p) => `${p.split(ROOT_PATH)?.[1].substring(1)}` || "";

  let owner, repo;

  try {
    owner = context?.repo?.owner;
    repo = context?.repo?.repo;
  } catch (_) {
    if (argv.owner) {
      owner = argv.owner;
    }
    if (argv.repo) {
      repo = argv.repo;
    }
  }

  if (!owner || !repo || !argv.token) {
    console.log(c.red("[💢 updater]"), "`owner`, `repo`, `token` are required.");
    process.exit(0);
  }

  let filename = UPDATER_JSON_PATH;
  if (argv.output) {
    filename = path.join(ROOT_PATH, argv.output);
    if (!/.json$/.test(filename)) {
      console.log(c.red("[💢 updater]"), c.yellow(filename), `The output file format must be json`);
      process.exit(0);
    }
  }
  if (!fs.existsSync(path.dirname(filename))) {
    fs.mkdirSync(path.dirname(filename), { recursive: true });
  }

  const options = { owner, repo };
  const github = getOctokit(argv.token);

  const { data: tags } = await github.rest.repos.listTags({
    ...options,
    per_page: 10,
    page: 1,
  });

  const tag = tags.find((t) => t.name.startsWith("v"));

  if (!tag) return;

  const { data: latestRelease } = await github.rest.repos.getReleaseByTag({
    ...options,
    tag: tag.name,
  });

  const updateData = {
    version: tag.name,
    pub_date: new Date().toISOString(),
    notes: latestRelease.body,
    platforms: {
      win64: { signature: "", url: "" }, // compatible with older formats
      linux: { signature: "", url: "" }, // compatible with older formats
      darwin: { signature: "", url: "" }, // compatible with older formats
      "darwin-aarch64": { signature: "", url: "" },
      "darwin-x86_64": { signature: "", url: "" },
      "linux-x86_64": { signature: "", url: "" },
      "windows-x86_64": { signature: "", url: "" },
    },
  };

  const setAsset = async (asset, reg, platforms) => {
    let sig = "";
    if (/.sig$/.test(asset.name)) {
      sig = await getSignature(asset.browser_download_url);
    }
    platforms.forEach((platform) => {
      if (reg.test(asset.name)) {
        // platform signature
        if (sig) {
          updateData.platforms[platform].signature = sig;
          return;
        }
        // platform url
        updateData.platforms[platform].url = asset.browser_download_url;
      }
    });
  };

  const promises = latestRelease.assets.map(async (asset) => {
    // windows
    await setAsset(asset, /.msi.zip/, ["win64", "windows-x86_64"]);

    // darwin
    await setAsset(asset, /.app.tar.gz/, ["darwin", "darwin-x86_64", "darwin-aarch64"]);

    // linux
    await setAsset(asset, /.AppImage.tar.gz/, ["linux", "linux-x86_64"]);
  });

  await Promise.allSettled(promises);

  fs.writeFileSync(filename, JSON.stringify(updateData, null, 2));
  console.log(c.green("[✨ updater]"), c.green(relativePath(filename)), "\n");
  console.log(c.yellow("*".repeat(20)));
  console.log(c.yellow("*"), c.blue("Edit `.github/workflows/release.yml > peaceiris/actions-gh-pages > publish_dir"));
  console.log(c.yellow("*"), c.gray("value:"), c.green(relativePath(path.dirname(filename))));
  console.log(c.yellow("*"), c.blue("Edit `tauri.conf.json > tauri > updater > endpoints`"));
  console.log(c.yellow("*"), c.gray("value:"), c.green(`https://${owner}.github.io/${repo}/${path.basename(filename)}`));
  console.log(c.yellow("*".repeat(20)));
})();

// get the signature file content
async function getSignature(url) {
  try {
    const response = await fetch(url, {
      method: "GET",
      headers: { "Content-Type": "application/octet-stream" },
    });
    return response.text();
  } catch (_) {
    return "";
  }
}
