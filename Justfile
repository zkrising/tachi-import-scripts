[private]
default:
	@just --choose

install:
	pnpm install

# Run the client interactively. Changes made to files will trigger reloads.
dev: install
	pnpm tauri dev

# Test that the client passes typechecking and linting.
test: install
	pnpm typecheck
	pnpm lint

build: install
	pnpm tauri build