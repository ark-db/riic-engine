<script lang="ts">
	import xmark from '$lib/images/red-xmark.svg';
	import { error } from '$lib/stores';
</script>

{#if $error}
	<div>
		<p>{$error}</p>
		<button on:click={error.clear}>
			<img src={xmark} alt="Delete button" width="20" height="20" />
		</button>
	</div>
{/if}

<style>
	div {
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
		line-height: 1.4;
	}
	button {
		position: absolute;
		top: -17px;
		right: -17px;
		border: 2px solid var(--salmon-strong);
		border-radius: 50%;
		padding: 2.5px;
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
	}
</style>
