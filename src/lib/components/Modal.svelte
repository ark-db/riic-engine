<script lang="ts">
	import { onMount } from 'svelte';

	export let onClose: () => void;

	let modal: HTMLDialogElement;
	onMount(() => modal.show());

	function handleMouseClose(event: MouseEvent) {
		let { button, target } = event;
		if (button === 0 && target instanceof Node && !modal.contains(target)) {
			onClose();
		}
	}

	function handleKeyboardClose(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			event.preventDefault();
			onClose();
		}
	}
</script>

<svelte:window on:keydown|trusted={handleKeyboardClose} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click|trusted={handleMouseClose}>
	<dialog bind:this={modal}>
		<slot {modal} />
	</dialog>
</div>

<style>
	div {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	div:before {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		content: '';
		background: rgba(0 0 0 / 0.7);
	}
	div:has(dialog[open]):before {
		animation: fade 0.2s ease-out;
	}
	dialog {
		box-shadow: 0 8px 8px rgba(0 0 0 / 0.5);
		border: none;
		border-radius: 1em;
		padding: 2em;
		background-color: var(--dark);
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	@keyframes zoom {
		from {
			scale: 0.95;
		}
		to {
			scale: 1;
		}
	}
</style>
