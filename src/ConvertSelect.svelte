<script lang="ts">
	import BeatorajaConvertComponent from "./Converters/BeatorajaConvertComponent.svelte";

	import Lr2ConvertComponent from "./Converters/LR2ConvertComponent.svelte";
	import UscConvertComponent from "./Converters/USCConvertComponent.svelte";
	import Divider from "./Divider.svelte";

	type Converters = "lr2DB" | "beatorajaDB" | "uscDB" | "";
	let mode: Converters = "";
</script>

<div class="col-12">
	<select bind:value={mode} class="form-control">
		<option value="">Select One...</option>
		<option value="lr2DB">Import Local LR2 Database</option>
		<option value="beatorajaDB">Import Local Beatoraja Database</option>
		<option value="uscDB">Import Local USC Database</option>
	</select>
</div>

<div class="col-12">
	{#if mode !== ""}
		<Divider />
	{/if}
	{#if mode === "lr2DB"}
		<Lr2ConvertComponent />
	{:else if mode === "beatorajaDB"}
		<BeatorajaConvertComponent />
	{:else if mode === "uscDB"}
		<UscConvertComponent />
	{/if}
</div>
