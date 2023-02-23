<script lang="ts">
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';
	import ColumnLines from './ColumnLines.svelte';

	let columnHeight: number;
	const columnGap = 36;
	let totalColumnWidth: number;
</script>

<ColumnLines {columnGap} {columnHeight} bind:totalColumnWidth />

<div
	class="facilities"
	style="--width: {totalColumnWidth + columnGap * 2}px;"
	bind:clientHeight={columnHeight}
>
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

	{#if $activeSave.data.layout.rr}
		<FacilityRow kind="meeting" />
	{/if}

	{#if $activeSave.data.layout.office}
		<FacilityRow kind="hire" />
	{/if}

	{#each $activeSave.data.layout.dorm as _}
		<FacilityRow kind="dormitory" />
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
