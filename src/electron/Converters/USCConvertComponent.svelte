<script lang="ts">
	import FileInputGroup from "../FileInputGroup.svelte";
	import { MakeIPCRequest } from "../ipc-utils";

	(async () => {
		const config = await MakeIPCRequest("CONFIG", null);

		dbPath = config.uscDB?.dbPath ?? "";
	})();

	let dbPath: string = "";
	let playtype: string = "";
</script>

<div class="mb-2">
	<FileInputGroup bind:value={dbPath} label="Score Database (maps.db)" />

	<div class="input-group">
		<div class="input-group-prepend">
			<div class="input-group-text">Input Device</div>
		</div>
		<select class="form-control" bind:value={playtype}>
			<option value="">Please Select...</option>
			<option value="Controller">Controller</option>
			<option value="Keyboard">Keyboard</option>
		</select>
	</div>
	<span class="text-warning"
		>Please select the input device you got your scores on. <br />
		<b>If this is not correct, you'll break your account!</b>
		<br />
		<b
			>Keyboard and Controller players get separate leaderboards. Lying about this will result
			in a permanent ban.</b
		>
	</span>
</div>

<div class="col-12 d-flex justify-content-center">
	{#if !dbPath || !playtype}
		<div class="btn btn-secondary" disabled>Convert & Import</div>
	{:else}
		<div
			class="btn btn-primary"
			on:click={async () => {
				const res = await MakeIPCRequest("USC_CONVERT", { dbPath, playtype });

				await MakeIPCRequest("IMPORT", res);
			}}
		>
			Convert & Import
		</div>
	{/if}
</div>
