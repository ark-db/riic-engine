<script lang="ts">
	import NumberInput from '$lib/components/NumberInput.svelte';
	import droneIcon from '$lib/images/ui/drones.avif';
	import InteractionBox from './InteractionBox.svelte';

	export let drones: number | undefined;
	export let onSetDrones: (drones: number) => void;

	let active: boolean;
	let box: HTMLDivElement;
	let template: HTMLDivElement;

	let submit: () => void;

	// Hide tooltip after it loses focus via clicking
	function handleClickOut({ x, y }: MouseEvent) {
		const target = document.elementFromPoint(x, y);

		if (!(target === box || template?.contains(target)) && active) {
			submit();
		}
	}
</script>

<svelte:window on:click|trusted={handleClickOut} />

<InteractionBox bind:active>
	<div
		slot="base"
		class="focus-template marker"
		class:active={drones && drones > 0}
		tabindex="0"
		role="button"
		bind:this={box}
	/>
	<div slot="menu" class="menu" bind:this={template}>
		<NumberInput
			desc="Drone count"
			min={0}
			max={999999}
			initial={drones ?? 0}
			placeholder="Drones"
			onValidInput={onSetDrones}
			errorMsg="The specified drone count should be a number from 0 to 999999"
			iconSrc={droneIcon}
			iconSize={32}
			bind:handleSubmit={submit}
		/>
	</div>
</InteractionBox>

<style>
	.marker {
		--focus-border-offset: -4px;
		--focus-border-radius: 4px;
		z-index: 1;
		/* --marker-width is defined in ./FacilityRow.svelte */
		width: var(--marker-width);
		/* --marker-height is defined in ./FacilityRow.svelte */
		height: var(--marker-height);
		background-color: rgb(221 160 221 / 0.4);
		&.active {
			background-color: rgb(221 160 221 / 0.7);
		}
	}
	.menu {
		--font-size: 1.3em;
		padding: 0.25em;
	}
</style>
