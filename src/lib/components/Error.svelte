<script lang="ts">
	import xmark from '$lib/images/red-xmark.svg';
	import { error } from '$lib/stores';

	export let msg: string;
</script>

<div class="container">
	<p>{msg}</p>
	<input
		type="image"
		src={xmark}
		alt="Delete button"
		height="20"
		width="20"
		on:click={error.clear}
	/>
</div>

<style>
	.container {
		position: fixed;
		z-index: 999;
		bottom: 5%;
		left: 50%;
		transform: translate(-50%, -50%);
		border: 2px solid var(--salmon-strong);
		border-radius: 1em;
		padding: 0 1em;
		background-color: var(--dark-strong);
	}
	p {
		color: var(--salmon-strong);
	}
	input {
		position: absolute;
		top: -15px;
		right: -15px;
		border: 2px solid var(--salmon-strong);
		border-radius: 50%;
		padding: 2.5px;
		background-color: var(--dark-strong);
	}
</style>
