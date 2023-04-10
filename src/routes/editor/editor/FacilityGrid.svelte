<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';

	export let rowWidth: number;
	export let rowHeight: number;
	export let columnWidth: number;
	export let gridHeight: number;

	$: rowGap = Math.max(6, Math.min(15, rowHeight ** 0.9 / 10));
</script>

<div
	class="facilities"
	style="--width: {rowWidth}px; --row-gap: {rowGap}px;"
	bind:clientHeight={gridHeight}
>
	<FacilityRow kind="control" {rowHeight} {columnWidth} bind:room={$activeSave.layout.cc} />

	{#each $activeSave.layout.tp as room}
		<FacilityRow kind="trading" {rowHeight} {columnWidth} bind:room />
	{/each}

	{#each $activeSave.layout.fac as room}
		<FacilityRow kind="manufacture" {rowHeight} {columnWidth} bind:room />
	{/each}

	{#each $activeSave.layout.pp as room}
		<FacilityRow kind="power" {rowHeight} {columnWidth} bind:room />
	{/each}

	{#if $activeSave.layout.rr.level > 0}
		<FacilityRow kind="meeting" {rowHeight} {columnWidth} bind:room={$activeSave.layout.rr} />
	{/if}

	{#if $activeSave.layout.office.level > 0}
		<FacilityRow kind="hire" {rowHeight} {columnWidth} bind:room={$activeSave.layout.office} />
	{/if}

	{#each $activeSave.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" {rowHeight} {columnWidth} bind:room />
		{/if}
	{/each}
</div>

<style>
	.facilities {
		width: var(--width);
		margin-top: 0.75em;
		padding-right: 1em;
		display: flex;
		flex-direction: column;
		row-gap: var(--row-gap);
	}
</style>
