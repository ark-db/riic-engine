<script lang="ts">
	import { activeSave } from '$lib/stores';

	export let columnWidth: number;
	export let gridWidth: number;
	export let gridHeight: number;
</script>

<div
	class="markers"
	style="--column-width: {columnWidth}px; --column-height: {gridHeight}px;"
	bind:clientWidth={gridWidth}
>
	{#each { length: $activeSave.data.maxShift } as _, i}
		<div class="column-marker">
			<p>{i + 1}</p>
			<div class="column-line" />
		</div>
	{/each}
</div>

<style>
	.markers {
		--label-border-width: 1px;
		--label-padding: calc(0.35em - var(--label-border-width));
		position: absolute;
		top: calc(0.25em - var(--label-padding) * 2);
		left: calc(3.5em + var(--column-width));
		display: flex;
		column-gap: var(--column-width);
	}
	.column-marker {
		z-index: 1;
		width: 1em;
		display: flex;
		flex-direction: column;
		align-items: center;
		row-gap: 0.25em;
	}
	p {
		z-index: 2;
		width: 1.75em;
		position: sticky;
		top: calc(0px - 0.575em - var(--label-padding) - var(--label-border-width));
		margin: 0;
		border: var(--label-border-width) solid var(--dark-mild);
		padding: var(--label-padding);
		background-color: black;
		cursor: default;
		color: var(--light-strong);
		font-size: 0.8em;
		display: flex;
		justify-content: center;
	}
	.column-line {
		width: 1px;
		height: var(--column-height);
		background-color: var(--light);
		opacity: 0.6;
	}
</style>
