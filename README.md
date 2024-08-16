# OllamaOne

Ollama UI App

## Features

- [x] Generate a chat completion
- [x] List Local Models
- [x] Delete a Model
- [x] Pull a Model

- config

```bash
sea-orm-cli migrate generate create_config_table

sea-orm-cli migrate fresh

rm -rf entity/src/*

sea-orm-cli generate entity -o entity/src --lib --with-serde both
```

- chat

```bash
sea-orm-cli migrate generate create_chat_table

sea-orm-cli migrate fresh

rm -rf entity/src/*

sea-orm-cli generate entity -o entity/src --lib --with-serde both
```

## macOS 编译步骤

```

export TAURI_PRIVATE_KEY="content of the generated key"
export TAURI_KEY_PASSWORD="password"
```

- aarch64

```
cd src-tauri
cargo build --release --target aarch64-apple-darwin
cd ..


yarn tauri build --target aarch64-apple-darwin

```

- x86_64

```
cargo build --release --target x86_64-apple-darwin
cd ..
yarn tauri build --target x86_64-apple-darwin
```

## Install the dependencies

```bash
yarn
# or
npm install
```

### Start the app in development mode (hot-code reloading, error reporting, etc.)

```bash
quasar dev
```

### Lint the files

```bash
yarn lint
# or
npm run lint
```

### Format the files

```bash
yarn format
# or
npm run format
```

### Build the app for production

```bash
quasar build
```

### Customize the configuration

See [Configuring quasar.config.js](https://v2.quasar.dev/quasar-cli-vite/quasar-config-js).
