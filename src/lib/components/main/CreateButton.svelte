<script lang="ts">
	import { tooltip } from '@tooltip';
	import addFile from '$lib/images/file-add.svg';
</script>

<input
	type="image"
	src={addFile}
	alt="Create new setup"
	width="25"
	height="25"
	title="Create new setup"
	use:tooltip
	on:click
/>

<style>
	input {
		border-radius: 5px;
		padding: 7.5px;
		transition: background-color 0.2s;
	}
	input:hover {
		background-color: var(--dark-mild);
		transition: background-color 0.1s;
	}
</style>
