<script lang="ts">
	import FileInputGroup from "../FileInputGroup.svelte";
	import { MakeIPCRequest } from "../ipc-utils";

	(async () => {
		const config = await MakeIPCRequest("CONFIG", null);

		dbPath = config.uscDB?.dbPath ?? "";
	})();

	let dbPath: string = "";
</script>

<div class="mb-2">
	<FileInputGroup bind:value={dbPath} label="Score Database (maps.db)" />
</div>

<div class="col-12 d-flex justify-content-center">
	{#if !dbPath}
		<div class="btn btn-secondary" disabled>Convert & Import</div>
	{:else}
		<div
			class="btn btn-primary"
			on:click={async () => {
				const res = await MakeIPCRequest("USC_CONVERT", dbPath);

				await MakeIPCRequest("IMPORT", res);
			}}
		>
			Convert & Import
		</div>
	{/if}
</div>
