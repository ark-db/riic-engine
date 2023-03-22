<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';

	export let rowWidth: number;
	export let rowHeight: number;
	export let gridHeight: number;

	$: rowGap = Math.max(6, Math.min(15, rowHeight ** 0.9 / 10));
</script>

<div
	class="facilities"
	style="--width: {rowWidth}px; --row-gap: {rowGap}px;"
	bind:clientHeight={gridHeight}
>
	<FacilityRow kind="control" height={rowHeight} bind:room={$activeSave.data.layout.cc} />

	{#each $activeSave.data.layout.tp as room}
		<FacilityRow kind="trading" height={rowHeight} bind:room />
	{/each}

	{#each $activeSave.data.layout.fac as room}
		<FacilityRow kind="manufacture" height={rowHeight} bind:room />
	{/each}

	{#each $activeSave.data.layout.pp as room}
		<FacilityRow kind="power" height={rowHeight} bind:room />
	{/each}

	{#if $activeSave.data.layout.rr.level > 0}
		<FacilityRow kind="meeting" height={rowHeight} bind:room={$activeSave.data.layout.rr} />
	{/if}

	{#if $activeSave.data.layout.office.level > 0}
		<FacilityRow kind="hire" height={rowHeight} bind:room={$activeSave.data.layout.office} />
	{/if}

	{#each $activeSave.data.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" height={rowHeight} bind:room />
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
