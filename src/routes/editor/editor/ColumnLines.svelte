<script lang="ts">
	import { lastShiftIndex } from '$lib/stores';
</script>

<div class="markers">
	{#each { length: $lastShiftIndex } as _, i}
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
		left: calc(1.75em + 22px + 36px);
		overflow: hidden;
		display: flex;
		column-gap: 36px;
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
		background-color: var(--light-strong);
		opacity: 0.5;
	}
</style>
