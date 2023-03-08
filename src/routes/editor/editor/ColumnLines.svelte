<script lang="ts">
	import { lastColumnNumber } from '$lib/stores';
	import { gridHeight } from './FacilityGrid.svelte';

	export let columnGap: number;
	export let totalColumnWidth: number;
</script>

<div
	class="markers"
	style="--column-gap: {columnGap}px; --column-height: {$gridHeight}px;"
	bind:clientWidth={totalColumnWidth}
>
	{#each { length: $lastColumnNumber } as _, i}
		<div class="column-marker">
			{i + 1}
			<div class="column-line" />
		</div>
	{/each}
</div>

<style>
	.markers {
		z-index: 1;
		position: absolute;
		top: 0.35em;
		left: calc(3.5em + var(--column-gap));
		overflow: hidden;
		display: flex;
		column-gap: var(--column-gap);
	}
	.column-marker {
		cursor: default;
		width: 1em;
		color: var(--light-strong);
		font-size: 0.8em;
		display: flex;
		flex-direction: column;
		align-items: center;
		row-gap: 0.25em;
	}
	.column-line {
		width: 1px;
		height: var(--column-height);
		background-color: var(--light);
		opacity: 0.6;
	}
</style>
