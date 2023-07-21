<script lang="ts">
	import { tooltip } from '$lib/tooltip';

	export let desc: string;
	export let onClick: () => void;

	let buttonWidth: number;
	let buttonHeight: number;

	// Reassigning `refreshState` forces element refresh via the {#key} block, since {} !== {}
	let refreshState: Record<string, never> = {};

	// Force element refresh whenever `desc` changes
	$: desc, (refreshState = {});

	$: small = Math.min(buttonWidth, buttonHeight) < 32;
</script>

{#key refreshState}
	<button
		class="focus-template"
		class:small
		aria-label={desc}
		bind:clientWidth={buttonWidth}
		bind:clientHeight={buttonHeight}
		use:tooltip={desc}
		on:click|trusted={onClick}
	>
		<slot />
	</button>
{/key}

<style>
	button {
		--focus-border-offset: -4px;
	}
	button.small {
		--focus-border-offset: -8px;
	}
</style>
