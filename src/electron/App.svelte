<script lang="ts">
	import { Body } from "node-fetch";
	import { GetTachiConfig } from "../node/util/tachi-info";
	const TachiConfig = GetTachiConfig();

	import Console from "./Console.svelte";
	import ContentMain from "./ContentMain.svelte";
	import ConvertSelect from "./ConvertSelect.svelte";
	import Divider from "./Divider.svelte";
	import { MakeIPCRequest } from "./ipc-utils";
</script>

<div class="container mt-4">
	<div class="row">
		<div class="col-12">
			<h1>{TachiConfig.name} Import Scripts</h1>
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
		{#await MakeIPCRequest("CONFIG", null)}
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
