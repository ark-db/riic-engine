<script lang="ts">
	import { lastColumnNumber } from '$lib/stores';

	export let columnGap: number;
	export let columnHeight: number;
	export let totalColumnWidth: number;
</script>

<div
	class="markers"
	style="--column-gap: {columnGap}px; --column-height: {columnHeight}px;"
	bind:clientWidth={totalColumnWidth}
>
	{#each { length: $lastColumnNumber } as _, i}
		<div class="column-marker">
			{i + 1}
			<div class="column-line" />
		</div>
	{/each}
</div>

<style>
	.markers {
		z-index: 1;
		position: absolute;
		top: 0.35em;
		/* padding from navbar
            + facility row edge padding
            + facility icon width
            + width of one-column wide gap */
		left: calc(0.75em + 1em + 22px + var(--column-gap));
		overflow: hidden;
		display: flex;
		column-gap: var(--column-gap);
	}
	.column-marker {
		color: var(--light-strong);
		font-size: 0.8em;
		display: flex;
		flex-direction: column;
		align-items: center;
		row-gap: 0.25em;
	}
	.column-line {
		height: var(--column-height);
		width: 1.5px;
		background-color: var(--light);
		opacity: 0.5;
	}
</style>
