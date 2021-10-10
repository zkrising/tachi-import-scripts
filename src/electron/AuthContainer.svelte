<script lang="ts">
	import TachiConfig from "../node/util/tachi-info";

	export let value: string | null;
	export let CheckAuthAgain: () => unknown;

	let show = false;
</script>

<div class="row justify-content-center">
	{#if show}
		<div class="col-8">
			<div class="input-group">
				<div class="input-group-prepend">
					<span class="input-group-text">Auth Key</span>
				</div>
				<input class="form-control" bind:value />
			</div>
			{#if value?.length !== 40 && value?.length !== 0}
				<span class="text-danger"
					>An auth key is exactly 40 characters long. Are you sure this is right?</span
				>
			{/if}
		</div>
		<div class="col-2">
			<a
				class="btn btn-info"
				href={TachiConfig.clientUrl + "/oauth/manual-apikey/" + TachiConfig.clientID}
				target="_blank">Get Auth Token</a
			>
		</div>
		<div class="col-2">
			<button class="btn btn-success" on:click={CheckAuthAgain}>Check Auth Again</button>
		</div>
	{:else}
		<button class="btn btn-danger" on:click={() => (show = true)}>Show Auth Key Field</button>
	{/if}
</div>
