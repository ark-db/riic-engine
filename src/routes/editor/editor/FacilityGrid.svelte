<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';

	export let gridHeight: number;
	export let rowWidth: number;
	export let rowHeight: number;

	$: rowGap = Math.max(6, Math.min(15, rowHeight ** 0.9 / 10));
</script>

<div
	class="facilities"
	style="--row-width: {rowWidth}px; --row-height: {rowHeight}px; --row-gap: {rowGap}px;"
	bind:clientHeight={gridHeight}
>
	<FacilityRow kind="control" bind:room={$activeSave.layout.cc} />

	{#each $activeSave.layout.tp as room}
		<FacilityRow kind="trading" bind:room />
	{/each}

	{#each $activeSave.layout.fac as room}
		<FacilityRow kind="manufacture" bind:room />
	{/each}

	{#each $activeSave.layout.pp as room}
		<FacilityRow kind="power" bind:room />
	{/each}

	{#if $activeSave.layout.rr.level > 0}
		<FacilityRow kind="meeting" bind:room={$activeSave.layout.rr} />
	{/if}

	{#if $activeSave.layout.office.level > 0}
		<FacilityRow kind="hire" bind:room={$activeSave.layout.office} />
	{/if}

	{#each $activeSave.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" bind:room />
		{/if}
	{/each}
</div>

<style>
	.facilities {
		width: var(--row-width);
		/* --shift-box-focus-border-offset is defined in ./+page.svelte */
		margin-top: calc(1em - var(--shift-box-focus-border-offset) * 2);
		padding-right: 2em;
		display: flex;
		flex-direction: column;
		row-gap: var(--row-gap);
	}
</style>
