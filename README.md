# Tachi Import Scripts

Import scripts for importing local database files to Tachi.

## Supports

- LR2
- Beatoraja
- unnamed_sdvx_clone

## How do I use it?

Acquire the build you want from the sidebar.

## Development Info

This project uses Svelte and Electron. To anyone who is going to complain about Electron performance, you are free to rewrite this for all operating systems in native UI toolkits.

```
yarn install
yarn start
```

If you get a `NODE_MODULE_VERSION 93 -> 98` related error, follow these steps:

```
1. npx electron-rebuild
2. yarn start

If it worked,
3. Profit
Else
3. Cry
```
