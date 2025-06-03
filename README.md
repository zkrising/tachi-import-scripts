# Tachi Import Scripts

Import scripts for importing local database files to Tachi.

## Supports

-   LR2
-   Beatoraja
-   unnamed_sdvx_clone

## How do I use it?

Acquire the build you want from the sidebar.

## Development Info

Use `just` to see what you can do.

The default CI setup does not compile a working Appimage due to a bug in Tauri or webkitgtk.
Use `NO_STRIP=true just build` to get a working Appimage.
