<script lang="ts">
	export let value: string;
	export let label: string;
</script>

<div class="input-group">
	<div class="input-group-prepend">
		<div class="input-group-text">
			{label}
		</div>
	</div>
	<input type="text" bind:value class="form-control" />
</div>
