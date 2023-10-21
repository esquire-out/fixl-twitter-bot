# fixl-twitter [![Rust Build and Test](https://github.com/esquire-out/fixl-twitter-bot/actions/workflows/rust.yml/badge.svg)](https://github.com/esquire-out/fixl-twitter-bot/actions/workflows/rust.yml)
A simple discord bot written in rust to modify urls in order to embed both twitter.com and x.com links properly.

# Setting up
At the moment there are no pre-compiled binaries so it must be compiled from source.
First install Rust if it's not installed already from [Rustup](https://rustup.rs/) using this command and follow the instructions.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repository using `git clone` or download it as a .zip and extract it.
```bash
git clone https://github.com/esquire-out/fixl-twitter-bot.git
```

check Cargo's version and Rust's compiler version to make sure they have been properly installed.
```bash
Cargo --version
```
```bash
rustc --version
```

with rust installed and the repository cloned **open a terminal in the directory** or **cd into the directory** in which the source files are located.
Should be `fixl-twitter-bot/`, compile the source code into a binary (might take a bit) to be able to run the bot.
```bash
cargo build --release
```
## Getting the bot Running
Once the binary is built export the discord token of the bot you which to connect to the shell env using `export` and replacing `YOURTOKENHERE` with the bot token.
```bash
export DISCORD_TOKEN=YOURTOKENHERE
```
after that run the executable located at the `fixl-twitter-bot/target/release/` with the name of `fix-twitterlinks-bot`.

## Arguments
run the bot with the argument `--skip-validation` to disabled URL validation, speeds up the process but it means the bot will resend invalid links that embed to nothing. Example:
```bash
./fix-twitterlinks-bot --skip-validation
```
