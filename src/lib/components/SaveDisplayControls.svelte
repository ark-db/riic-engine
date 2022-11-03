<script lang="ts">
	import { tooltip } from '@tooltip';
	import addFile from '$lib/images/file-add.svg';
</script>

<div>
	<input
		type="image"
		src={addFile}
		alt="Create new setup"
		width="30"
		height="30"
		title="Create new setup"
		use:tooltip
		on:click
	/>
</div>

<style>
	div {
		margin: 1em 0;
		display: flex;
		align-items: center;
	}
	input {
		border-radius: 7.5px;
		padding: 7.5px;
		transition: background-color 0.2s;
	}
	input:hover {
		background-color: var(--dark-mild);
		transition: background-color 0.1s;
	}
</style>
