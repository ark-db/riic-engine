<script context="module" lang="ts">
	import { writable, readonly } from 'svelte/store';

	const rowLengthStore = writable<number>();
	export const rowLength = readonly(rowLengthStore);
</script>

<script lang="ts">
	import { lastColumnNumber } from '$lib/stores';
	import { gridHeight } from './FacilityGrid.svelte';

	export let columnGapScale: number;

	const baseColumnGap = 40;
	$: columnGap = columnGapScale * baseColumnGap;

	let totalColumnWidth: number;
	$: $rowLengthStore = totalColumnWidth + columnGap + 37;
</script>

<div
	class="markers"
	style="--column-gap: {columnGap}px; --column-height: {$gridHeight}px;"
	bind:clientWidth={totalColumnWidth}
>
	{#each { length: $lastColumnNumber } as _, i}
		<div class="column-marker">
			<p>{i + 1}</p>
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
		width: 1em;
		color: var(--light-strong);
		font-size: 0.8em;
		display: flex;
		flex-direction: column;
		align-items: center;
		row-gap: 0.25em;
	}
	p {
		cursor: default;
		margin: 0;
	}
	.column-line {
		width: 1px;
		height: var(--column-height);
		background-color: var(--light);
		opacity: 0.6;
	}
</style>
