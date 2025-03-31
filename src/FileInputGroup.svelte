<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";

	export let value: string;
	export let label: string;

	let inputControl: HTMLInputElement;
</script>

<div class="form-group">
	<label class="form-label" for="file-control">
		{label}
	</label>
	<input
		bind:this={inputControl}
		type="button"
		name="file-control"
		class="form-control"
		hidden
		on:click={async () => {
			let filepath = await open();
			if (!filepath) {
				return;
			}

			value = filepath;
		}}
	/>
	<br />
	<div class="text-left">
		<button
			class={`btn ${value ? "btn-secondary" : "btn-primary"}`}
			on:click={() => {
				inputControl.click();
			}}>Select File</button
		>
		<span>
			Selected File: {value}
		</span>
	</div>
</div>
