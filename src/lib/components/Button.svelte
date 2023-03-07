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
	$: small = Math.min(buttonWidth, buttonHeight) < 32;

	function handleClick() {
		refreshState = {};
		onClick();
	}
</script>

{#key refreshState}
	<button
		class="focus-template"
		class:small
		use:tooltip={desc}
		on:click|trusted={handleClick}
		bind:clientWidth={buttonWidth}
		bind:clientHeight={buttonHeight}
	>
		<slot />
	</button>
{/key}

<style>
	button {
		--focus-border-offset: -4px;
		margin: 0;
		border: none;
		padding: 0;
		background: none;
	}
	.small {
		--focus-border-offset: -8px;
	}
</style>
