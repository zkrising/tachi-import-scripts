<script lang="ts">
	const ipcRenderer = require("electron").ipcRenderer;

	let logs: any[] = [];

	let cns: HTMLDivElement;

	ipcRenderer.on("logger", (_, e) => {
		logs.push(e);

		logs = logs;

		(cns.lastChild! as HTMLDivElement).scrollIntoView({ behavior: "smooth" });
	});
</script>

<div bind:this={cns} class="container console" id="console">
	{#each logs as log}
		<div class={`log-entry log-level-${log.level}`}>
			[{new Date(log.timestamp).toLocaleTimeString()}] {log.level.toUpperCase()}: {log.message}
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
