<script context="module" lang="ts">
	import { writable, readonly } from 'svelte/store';

	const gridHeightStore = writable<number>();
	export const gridHeight = readonly(gridHeightStore);

	const rowHeightStore = writable<number>();
	export const rowHeight = readonly(rowHeightStore);
</script>

<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';
	import { rowLength } from './ColumnLines.svelte';

	export let rowHeightScale: number;

	const baseRowHeight = 80;
	$: $rowHeightStore = rowHeightScale * baseRowHeight;
</script>

<div class="facilities" style="--width: {$rowLength}px;" bind:clientHeight={$gridHeightStore}>
	<FacilityRow kind="control" />

	{#each $activeSave.data.layout.tp as _}
		<FacilityRow kind="trading" />
	{/each}

	{#each $activeSave.data.layout.fac as _}
		<FacilityRow kind="manufacture" />
	{/each}

	{#each $activeSave.data.layout.pp as _}
		<FacilityRow kind="power" />
	{/each}

	{#if $activeSave.data.layout.rr.level > 0}
		<FacilityRow kind="meeting" />
	{/if}

	{#if $activeSave.data.layout.office.level > 0}
		<FacilityRow kind="hire" />
	{/if}

	{#each $activeSave.data.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" />
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
