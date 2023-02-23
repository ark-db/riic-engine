<script lang="ts">
	import { activeSave } from '$lib/stores';
	import RowWrapper from './RowWrapper.svelte';
	import ColumnLines from './ColumnLines.svelte';

	let columnHeight: number;
</script>

<ColumnLines --column-height={`${columnHeight}px`} />

<div class="facilities" bind:clientHeight={columnHeight}>
	<RowWrapper kind="control" />

	{#each $activeSave.data.layout.tp as _}
		<RowWrapper kind="trading" />
	{/each}

	{#each $activeSave.data.layout.fac as _}
		<RowWrapper kind="manufacture" />
	{/each}

	{#each $activeSave.data.layout.pp as _}
		<RowWrapper kind="power" />
	{/each}

	{#if $activeSave.data.layout.rr}
		<RowWrapper kind="meeting" />
	{/if}

	{#if $activeSave.data.layout.office}
		<RowWrapper kind="hire" />
	{/if}

	{#each $activeSave.data.layout.dorm as _}
		<RowWrapper kind="dormitory" />
	{/each}
</div>

<style>
	.facilities {
		margin-top: 0.75em;
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
</style>
