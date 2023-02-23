<script lang="ts">
	export let onClose: () => void;

	function handleMouseClose(event: MouseEvent) {
		if (event.button === 0) onClose();
	}

	function handleKeyboardClose(event: KeyboardEvent) {
		if (event.key === 'Escape') onClose();
	}
</script>

<svelte:window on:keydown|trusted|preventDefault={handleKeyboardClose} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="bg" on:click|trusted={handleMouseClose} />

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
		background-color: rgb(0 0 0 / 0.7);
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
