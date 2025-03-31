<script lang="ts">
	import { listen } from "@tauri-apps/api/event";

	let logs: any[] = [];

	let cns: HTMLDivElement;

	listen("log", (e) => {
		logs.push(e.payload);

		console.log(logs);

		logs = logs;

		try {
			(cns.lastChild! as HTMLDivElement).scrollIntoView({ behavior: "smooth" });
		} catch {}
	});
</script>

<div bind:this={cns} class="container console" id="console">
	{#each logs as log}
		<div class={`log-entry log-level-${log.level}`}>
			[{new Date(log.timestamp).toLocaleTimeString()}] {log.level.toUpperCase()}: {log.msg}
		</div>
	{/each}
</div>

<style>
	.console {
		background-color: black;
		height: 200px;
		overflow-y: scroll;
		overscroll-behavior-y: contain;
		scroll-snap-type: y proximity;
		width: 100%;
	}

	.console > div:last-child {
		scroll-snap-align: end;
	}

	.log-entry {
		background-color: black;
		color: white;
		font-family: monospace;
		word-wrap: break-word;
	}

	.log-level-info {
		color: cyan;
	}

	.log-level-warn {
		color: yellow;
	}

	.log-level-error {
		color: red;
	}

	.log-level-severe {
		background-color: white;
		color: red;
	}

	.log-level-crit {
		background-color: red;
		color: white;
	}
</style>
