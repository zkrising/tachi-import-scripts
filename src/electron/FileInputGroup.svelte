<script lang="ts">
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
		type="file"
		name="file-control"
		class="form-control"
		hidden
		on:change={(e) => {
			// @ts-expect-error temp
			value = e.target.files[0].path;
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
