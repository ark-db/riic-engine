<script lang="ts">
	import { activeSave } from '$lib/stores';

	export let gridWidth: number;
	export let gridHeight: number;
</script>

<div class="markers" style="--column-height: {gridHeight}px;" bind:clientWidth={gridWidth}>
	<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
	{#each { length: $activeSave.maxShift } as _, i}
		<div class="column-marker">
			<p>{i + 1}</p>
			<div class="column-line" />
		</div>
	{/each}
</div>

<style>
	.markers {
		--label-border-width: 1px;
		--label-padding: 0.35em;
		--column-marker-width: 1em;
		position: absolute;
		top: 0;
		/* facility icon size = 24px */
		/* --facility-edge-padding is defined in ./+page.svelte */
		--facility-edge-width: calc(24px + var(--facility-edge-padding) * 2);
		--column-marker-offset: calc((2em - var(--column-marker-width)) / 2);
		/* --page-padding is defined in ../+layout.svelte */
		/* --column-width is defined in ./+page.svelte */
		left: calc(
			var(--facility-edge-width) + var(--page-padding) + var(--column-width) +
				var(--column-marker-offset)
		);
		display: flex;
		column-gap: var(--column-width);
	}
	.column-marker {
		width: var(--column-marker-width);
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	p {
		z-index: 2;
		width: 1.75em;
		position: sticky;
		border: var(--label-border-width) solid var(--dark-mild);
		padding: var(--label-padding);
		background-color: black;
		color: var(--light-strong);
		font-size: 0.8em;
		font-weight: 500;
		display: flex;
		justify-content: center;
	}
	.column-line {
		z-index: 1;
		position: relative;
		/* --shift-box-focus-border-offset is defined in ./+page.svelte */
		top: calc(-1.5px - var(--shift-box-focus-border-offset) * 2);
		width: 1px;
		height: var(--column-height);
		background-color: var(--light);
	}
</style>
