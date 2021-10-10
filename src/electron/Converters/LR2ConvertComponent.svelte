<script lang="ts">
	import FileInputGroup from "../FileInputGroup.svelte";
	import { MakeIPCRequest } from "../ipc-utils";

	(async () => {
		const config = await MakeIPCRequest("CONFIG", null);

		scorePath = config.lr2DB?.scorePath ?? "";
		chartPath = config.lr2DB?.chartPath ?? "";
	})();

	let scorePath: string = "";
	let chartPath: string = "";
</script>

<div class="mb-2">
	<FileInputGroup bind:value={scorePath} label="Score Database (<username>.db)" />
</div>
<div class="mb-2">
	<FileInputGroup bind:value={chartPath} label="Chart Database (song.db)" />
</div>
<div class="alert alert-secondary">
	LR2 has a bug where, in certain scenarios, you can get an Auto Scratch EASY CLEAR with no
	indication the score was performed with Auto Scratch. The import tool <b>CANNOT</b> tell when
	this has happened, and this may result in some invalid imports. You can read more about this
	<a
		class="text-danger"
		target="_blank"
		href="http://tachi.rtfd.io/user/score-oddities/#lr2-auto-scratch-easy-clear">Here</a
	>.
</div>

<div class="col-12 d-flex justify-content-center">
	{#if !scorePath || !chartPath}
		<div class="btn btn-secondary" disabled>Convert & Import</div>
	{:else}
		<div
			class="btn btn-primary"
			on:click={async () => {
				const res = await MakeIPCRequest("LR2_CONVERT", { scorePath, chartPath });

				for (const bm of res) {
					const res2 = await MakeIPCRequest("IMPORT", bm);
				}
			}}
		>
			Convert & Import
		</div>
	{/if}
</div>
