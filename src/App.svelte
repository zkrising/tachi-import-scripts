<script lang="ts">
	const TachiConfig = GetTachiConfig();

	import Console from "./Console.svelte";
	import ContentMain from "./ContentMain.svelte";
	import Divider from "./Divider.svelte";
	import { MakeIPCRequest } from "./ipc-utils";
	import { GetTachiConfig } from "./tachi-info";
</script>

<div class="container mt-4">
	<div class="row">
		<div class="col-12">
			<h1>{TachiConfig.name} Import Scripts (v2.1.0)</h1>
		</div>
		<div class="col-12">
			<h4>
				This is a thin client for importing various files on your PC to {TachiConfig.name}.
			</h4>
		</div>
	</div>
	<div class="row">
		<div class="col-12">
			<Divider className="my-2" />
		</div>
	</div>
	<div class="row">
		{#await MakeIPCRequest("config", {})}
			Loading...
		{:then config}
			<ContentMain {config} />
		{/await}
	</div>
	<div class="row">
		<div class="col-12">
			<Divider className="my-2" />
		</div>
	</div>
	<div class="row">
		<Console />
	</div>
</div>
