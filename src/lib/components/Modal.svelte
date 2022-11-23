<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{ close: Record<string, never> }>();

	function handleClose(event: MouseEvent | KeyboardEvent) {
		if (event instanceof MouseEvent || event.key === 'Escape') {
			dispatch('close');
		}
	}
</script>

<div class="bg" on:click={handleClose} on:keydown={handleClose} />
<section>
	<slot />
</section>

<style>
	.bg {
		position: fixed;
		z-index: 99;
		top: 0;
		left: 0;
		height: 100vh;
		width: 100vw;
		background-color: rgba(0, 0, 0, 0.7);
	}
	section {
		position: fixed;
		z-index: 100;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		border-radius: 1em;
		padding: 2em;
		background-color: var(--dark);
	}
</style>
