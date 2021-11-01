import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";
import { sveltePreprocess } from "svelte-preprocess/dist/autoProcess";
import typescript from "@rollup/plugin-typescript";
import json from "@rollup/plugin-json";
import nodePolyfill from "rollup-plugin-polyfill-node";

const production = !process.env.ROLLUP_WATCH;

export default {
	input: "src/electron/main.ts",
	output: {
		sourcemap: true,
		format: "iife",
		name: "app",
		file: "dist/ui-bundle.js",
	},
	plugins: [
		svelte({
			// enable run-time checks when not in production
			emitCss: false,
			preprocess: sveltePreprocess(),
		}),

		json(),

		nodePolyfill(),

		typescript({ sourceMap: !production }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: ["svelte"],
		}),
		commonjs(),

		// In dev mode, call `npm run start` once
		// the bundle has been generated
		!production && serve(),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser(),
	],
	watch: {
		clearScreen: false,
	},
};

function serve() {
	let started = false;

	return {
		writeBundle() {
			if (!started) {
				started = true;

				// eslint-disable-next-line @typescript-eslint/no-var-requires
				require("child_process").spawn("npm", ["run", "start", "--", "--dev"], {
					stdio: ["ignore", "inherit", "inherit"],
					shell: true,
				});
			}
		},
	};
}
