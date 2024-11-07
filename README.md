# 🔎 twguessr
![Rust](https://img.shields.io/badge/Rust-3d1728?logo=rust&logoColor=white)
![Discord Bot](https://img.shields.io/badge/Discord_Bot-272a5b?logo=discord&logoColor=white)

## 📌 Info
- This is a discord bot for playing guessing game with teeworlds maps.
- Maps released from both kog and ddnet are supported. Players can choose between the two map pools.
- The code is probably very poor and crappy as I'm still learning Rust.

## 🎲 Game Rules
- Fastest user to send the map name (case insensitive) wins.
- There is a 90 second time limit.
- The answer can be revealed without waiting by skipping.
- Players can take the following hints in order: map rating, map creator and first letter of the answer.
> Everything here is subject to change. Feel free to provide feedback.

## ❄️ Nix Setup
This project uses devenv with pre-commit hooks for development.

Environment variables are loaded from the `.env` file.

- Activating the dev shell:
```bash
nix develop --impure
```
- Running the project and auto reloading on changes:
```bash
cargo watch --exec "run"
```
- Running pre-commit hooks:
```bash
pre-commit run
```
### Visual Studio Code
Make sure to load the dev shell with [direnv](https://marketplace.visualstudio.com/items?itemName=mkhl.direnv) or [Nix Environment Selector](https://marketplace.visualstudio.com/items?itemName=arrterian.nix-env-selector) for rust-analyzer to work.