<script lang="ts">
	// tippy.js tooltips are buggy when used on interactive elements such as <button>.
	// This custom button component resolves some of the issues by refreshing the element.

	import { tooltip } from '$lib/tooltip';

	export let desc: string;
	export let onClick: () => void;

	// Reassigning `refreshState` forces element refresh via the {#key} block, since {} !== {}
	let refreshState: Record<string, never> = {};

	// Force element refresh whenever `desc` changes
	$: desc, (refreshState = {});

	let buttonWidth: number;
	let buttonHeight: number;
	$: focusOffset = Math.min(buttonHeight, buttonWidth) < 32 ? -8 : -4;

	function handleClick() {
		refreshState = {};
		onClick();
	}
</script>

{#key refreshState}
	<button
		style="--border-offset: {focusOffset}px;"
		use:tooltip={desc}
		on:click|trusted={handleClick}
		bind:clientHeight={buttonHeight}
		bind:clientWidth={buttonWidth}
	>
		<slot />
	</button>
{/key}

<style>
	button {
		position: relative;
		margin: 0;
		border: none;
		padding: 0;
		background: none;
	}
	button:after {
		opacity: 0;
		transition: opacity 0.35s;
	}
	button:focus {
		outline: none;
	}
	button:focus:after {
		content: '';
		display: block;
		position: absolute;
		top: var(--border-offset);
		bottom: var(--border-offset);
		left: var(--border-offset);
		right: var(--border-offset);
		border-radius: 8px;
		border: 2.5px solid var(--blue-mild);
		opacity: 1;
	}
</style>
