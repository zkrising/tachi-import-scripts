import "svelte";
import App from "./App.svelte";

console.log("A.");

const app = new App({
	target: document.body,
});

export default app;
