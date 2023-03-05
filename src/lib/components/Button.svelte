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
		class="focus-template"
		style="--focus-border-offset: {focusOffset}px;"
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
		margin: 0;
		border: none;
		padding: 0;
		background: none;
	}
</style>
