<script context="module" lang="ts">
	import { writable, readonly } from 'svelte/store';

	const gridHeightStore = writable<number>();
	export const gridHeight = readonly(gridHeightStore);
</script>

<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';
	import { rowLength } from './ColumnLines.svelte';

	export let rowHeightScale: number;

	const baseRowHeight = 80;
	$: rowHeight = rowHeightScale * baseRowHeight;
</script>

<div class="facilities" style="--width: {$rowLength}px;" bind:clientHeight={$gridHeightStore}>
	<FacilityRow kind="control" height={rowHeight} />

	{#each $activeSave.data.layout.tp as _}
		<FacilityRow kind="trading" height={rowHeight} />
	{/each}

	{#each $activeSave.data.layout.fac as _}
		<FacilityRow kind="manufacture" height={rowHeight} />
	{/each}

	{#each $activeSave.data.layout.pp as _}
		<FacilityRow kind="power" height={rowHeight} />
	{/each}

	{#if $activeSave.data.layout.rr.level > 0}
		<FacilityRow kind="meeting" height={rowHeight} />
	{/if}

	{#if $activeSave.data.layout.office.level > 0}
		<FacilityRow kind="hire" height={rowHeight} />
	{/if}

	{#each $activeSave.data.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" height={rowHeight} />
		{/if}
	{/each}
</div>

<style>
	.facilities {
		width: var(--width);
		margin-top: 0.75em;
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
</style>
