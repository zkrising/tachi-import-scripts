<script lang="ts">
	import FileInputGroup from "../FileInputGroup.svelte";
	import { MakeIPCRequest } from "../ipc-utils";

	(async () => {
		const config = await MakeIPCRequest("config", {});

		scorePath = config.beatorajaDB?.scorePath ?? "";
		chartPath = config.beatorajaDB?.chartPath ?? "";
	})();

	let scorePath: string = "";
	let chartPath: string = "";
</script>

<div class="mb-2">
	<FileInputGroup bind:value={scorePath} label="Score Database (player/player1/score.db)" />
</div>
<div class="mb-2">
	<FileInputGroup bind:value={chartPath} label="Chart Database (songdata.db, NOT songinfo.db!)" />
</div>

<div class="col-12 d-flex justify-content-center">
	{#if !scorePath || !chartPath}
		<div class="btn btn-secondary" disabled>Convert & Import</div>
	{:else}
		<div
			class="btn btn-primary"
			on:click={async () => {
				const res = await MakeIPCRequest("beatoraja_convert", {
					opts: { scorePath, chartPath },
				});

				for (const bm of Object.values(res)) {
					if (!bm) {
						continue;
					}
					const res2 = await MakeIPCRequest("import", { bm });
				}
			}}
		>
			Convert & Import
		</div>
	{/if}
</div>
