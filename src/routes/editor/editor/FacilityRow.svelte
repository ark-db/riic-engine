<script lang="ts">
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import facilities from '$lib/data/facilities.json';
	import { activeSave } from '$lib/stores';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName, Facility, BoostFacility } from '$lib/types';
	import ProductBox from './ProductBox.svelte';

	export let kind: FacilityName;
	export let rowHeight: number;
	export let columnWidth: number;
	export let room: Facility | BoostFacility;

	const rowOpacity = 0.7;
	let { name, color } = facilities[kind];

	// Converts a hex triplet into the CSS rgb() format
	function hexToRgb(hex: string, alpha = 1.0): string {
		let r = parseInt(hex.slice(1, 3), 16),
			g = parseInt(hex.slice(3, 5), 16),
			b = parseInt(hex.slice(5, 7), 16);

		return `rgb(${r} ${g} ${b} / ${alpha})`;
	}
</script>

<div class="container" style="--color: {color}; --color-a: {hexToRgb(color, rowOpacity)}">
	<div class="edge" use:tooltip={name}>
		<FacilityIcon {kind} size={24} />
	</div>
	<div class="main" style="--height: {rowHeight}px; --column-width: {columnWidth}px;">
		{#if 'products' in room && (kind === 'manufacture' || kind === 'trading')}
			<div class="products">
				{#each { length: $activeSave.data.maxShift } as _, i}
					<ProductBox
						{kind}
						level={room.level}
						onAddProduct={(p) => console.log(`Added product ${p} at column ${i}`)}
					/>
				{/each}
			</div>
		{/if}
		<div class="chars">
			<!-- TODO -->
		</div>
	</div>
</div>

<style>
	.container {
		background-color: var(--color-a);
		display: flex;
	}
	.edge {
		padding: 0 calc(0.5em - 2px);
		background-color: var(--color);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.main {
		flex-grow: 1;
	}
	.products {
		/* --row-height is used in ./ProductBox.svelte */
		--row-height: calc(var(--height) / 5);
		--shadow-size: calc(var(--row-height) / 16);
		height: var(--row-height);
		box-shadow: calc(var(--shadow-size) / 2) var(--shadow-size) var(--shadow-size) rgb(0 0 0 / 0.5);
		background-color: var(--darkish);
		display: flex;
	}
	.chars {
		height: var(--height);
	}
</style>
