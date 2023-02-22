<script lang="ts">
	import { activeSave, lastShiftIndex } from '$lib/stores';
	import RowWrapper from './RowWrapper.svelte';
</script>

<div class="markers">
	{#each { length: $lastShiftIndex } as _, i}
		<div class="column-marker">
			{i}
			<div class="column-line" />
		</div>
	{/each}
</div>

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

<style>
	.markers {
		position: absolute;
		display: flex;
		column-gap: 24px;
	}
	.column-marker {
		width: 2px;
		background-color: var(--gray);
		height: 100%;
		/*float: left;*/
	}
</style>
