<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';
	import ColumnLines from './ColumnLines.svelte';
	import ZoomControls from './ZoomControls.svelte';

	let xScale: number;
	let yScale: number;

	const columnGap = 36;
	let totalColumnWidth: number;
	$: facilityRowWidth = totalColumnWidth + columnGap + 36;
	let columnHeight: number;
</script>

<ColumnLines {columnGap} {columnHeight} bind:totalColumnWidth />

<div class="facilities" style="--width: {facilityRowWidth}px;" bind:clientHeight={columnHeight}>
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

<ZoomControls bind:xScale bind:yScale />

<style>
	.facilities {
		width: var(--width);
		margin-top: 0.75em;
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
</style>
