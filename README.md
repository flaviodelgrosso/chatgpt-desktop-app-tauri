<p align="center">
  <img src="src-tauri/icons/icon.png" width="200"/>
</p>

# ChatGPT Desktop Application

Unofficial open source OpenAI ChatGPT desktop app for macOS, Windows, and Linux menubar using Tauri (<https://tauri.app/>) and Rust

![License](https://img.shields.io/badge/License-Apache%202-blue.svg)
[![ChatGPT downloads](https://img.shields.io/github/downloads/flaviodelgrosso/chatgpt-desktop-app-tauri/total.svg)](https://github.com/flaviodelgrosso/chatgpt-desktop-app-tauri/releases)

![screen](./screenshots/window.png)

## Global shortcuts

You can use Cmd+Shift+G (Mac) or Ctrl+Shift+G (Win) to quickly open it from anywhere.

## Updater

This application integrates the Tauri Updater, a tool built in Rust that allows for easy application updates. With the Tauri Updater, users can easily keep their application up to date with the latest bug fixes and features. The integration of the Tauri Updater ensures that users have a seamless update experience.

![screen](./screenshots/update.png)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Developing

```bash
yarn
yarn dev
```

## Building

```bash
yarn
yarn build
```
