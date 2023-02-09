# Contribution Guidelines

We would love for you to contribute to this project.
As a contributor, here are the guidelines we would like you to follow:

## Be Kind - Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) to help us keep this project open and inclusive.

<br/>

## Found a bug? Want a feature? - Submit an Issue

[Choose an issue template](https://github.com/flaviodelgrosso/chatgpt-desktop-app-tauri/issues/new/choose) to file a bug report / feature request.

<br/>

## Ready to contribute a Pull Request (PR)?

<br/>

### ▶ 1. First - [Search this repo for existing PRs](https://github.com/flaviodelgrosso/chatgpt-desktop-app-tauri/pulls)

Try to find an open or closed PR that relates to the change you want to introduce.

<br/>

### ▶ 2. **Before you start coding - [find](https://github.com/flaviodelgrosso/chatgpt-desktop-app-tauri/issues) / [create an issue](https://github.com/flaviodelgrosso/chatgpt-desktop-app-tauri/issues/new/choose)**

**Make sure there's an issue** describing the problem you're fixing, or documents the design for the feature you'd like to add.
Discussing the design up front helps to ensure that we're ready to accept your work.

**Don't waste your time working on code before you got a 👍 in an issue comment.**

<br/>

### ▶ 3. Fork the this repo and create a branch

- Hit the "Fork" button above this GitHub repository

- `git clone YOUR_FORK_URL`

- Create a new branch locally in your fork's repo

- `git checkout -b my-fix-branch master`

<br/>

### ▶ 4. Run the development server

- From the root of the project run `yarn install`.

- Then, in one terminal run `yarn dev`

<br/>

### ▶ 5. Push your branch to GitHub

```shell
git push origin my-fix-branch
```

### ▶ 6. Create a PR

In GitHub, create a pull request for `flaviodelgrosso/chatgpt-desktop-app-tauri:master`

Make sure you check the checkbox "Allow edits from maintainers"

If you need to update your PR for some reason -

- Make the required updates.

- Rebase your branch and force push to your GitHub repository (this will update your Pull Request):

  ```shell
  git rebase master -i
  git push -f
  ```

<br/>

### ▶ 7. After your PR is merged - delete your branches

After your pull request is merged, you can safely delete your branch and pull the changes from the main (upstream) repository:

- Delete the remote branch on GitHub either through the GitHub web UI or your local shell as follows:

  ```shell
  git push origin --delete my-fix-branch
  ```

- Check out the master branch:

  ```shell
  git checkout master -f
  ```

- Delete the local branch:

  ```shell
  git branch -D my-fix-branch
  ```

- Update your master with the latest upstream version:

  ```shell
  git pull --ff upstream master
  ```

<br/>

### Thanks for your contribution! 🙏
