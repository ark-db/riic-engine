<script lang="ts">
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import facilities from '$lib/data/facilities.json';
	import { activeSave, zoomScales } from '$lib/stores';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName, Facility, BoostFacility, Product } from '$lib/types';
	import BoostMarker from './BoostMarker.svelte';
	import ProductBox from './ProductBox.svelte';
	import ShiftBox from './ShiftBox.svelte';

	export let kind: FacilityName;
	export let room: Facility | BoostFacility;

	const rowOpacity = 0.7;
	const { name, color } = facilities[kind];

	const { xScale } = zoomScales;
	const baseBoostMarkerWidth = 8;
	$: boostMarkerWidth = $xScale ** 0.6 * baseBoostMarkerWidth;

	// Reassignment forces element refresh via the {#key} block, since {} !== {}
	let refreshBoostState: Record<string, never> = {};
	let refreshProductState: Record<string, never> = {};

	// Converts a hex triplet into the CSS rgb() format
	function hexToRgb(hex: string, alpha: number): string {
		const r = parseInt(hex.slice(1, 3), 16),
			g = parseInt(hex.slice(3, 5), 16),
			b = parseInt(hex.slice(5, 7), 16);

		return `rgb(${r} ${g} ${b} / ${alpha})`;
	}

	function getDrones(index: number) {
		return (room as BoostFacility).boosts[index];
	}

	function setDrones(drones: number, index: number) {
		(room as BoostFacility).boosts[index] = drones;
		refreshBoostState = {};
	}

	function getProduct(index: number) {
		return (room as BoostFacility).products[index];
	}

	function setProduct(product: Product, index: number) {
		(room as BoostFacility).products[index] = product;
		refreshProductState = {};
	}
</script>

<div class="container" style="--color: {color}; --color-a: {hexToRgb(color, rowOpacity)}">
	<div class="edge" use:tooltip={name}>
		<FacilityIcon {kind} size={24} />
	</div>
	<div class="main">
		{#if kind === 'trading' || kind === 'manufacture'}
			<div class="boosts" style="--marker-width: {boostMarkerWidth}px;">
				{#key refreshBoostState}
					<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
					{#each { length: $activeSave.maxShift } as _, i}
						<BoostMarker drones={getDrones(i)} onSetDrones={(drones) => setDrones(drones, i)} />
					{/each}
				{/key}
			</div>
			<div class="products">
				{#key refreshProductState}
					<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
					{#each { length: $activeSave.maxShift } as _, i}
						<ProductBox
							{kind}
							level={room.level}
							product={getProduct(i)}
							onSetProduct={(product) => setProduct(product, i)}
						/>
					{/each}
				{/key}
			</div>
		{/if}
		<div class="chars">
			<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
			{#each { length: $activeSave.maxShift } as _}
				<ShiftBox {kind} level={room.level} />
			{/each}
		</div>
	</div>
</div>

<style>
	.container {
		background-color: var(--color-a);
		display: flex;
	}
	.edge {
		/* --facility-edge-padding is defined in ./+page.svelte */
		padding: 0 var(--facility-edge-padding);
		background-color: var(--color);
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.main {
		/* --box-width is used in ./ProductBox.svelte and ./ShiftBox.svelte */
		/* --column-width is defined in ./+page.svelte */
		--box-width: calc(var(--column-width) + 1em);
		/* --product-row-height is used in ./ProductBox.svelte */
		/* --row-height is defined in ./+page.svelte */
		--product-row-height: calc(var(--row-height) / 5);

		flex-grow: 1;
		position: relative;
	}
	.boosts {
		/* --marker-height is used in ./BoostMarker.svelte */
		--marker-height: calc(var(--row-height) + var(--product-row-height));
		position: absolute;
		top: 0;
		height: var(--marker-height);
		margin-left: calc(var(--column-width) - var(--marker-width) / 2 + 16px);
		display: flex;
		column-gap: calc(var(--column-width) - var(--marker-width) + 16px);
	}
	.products {
		--shadow-size: calc(var(--product-row-height) / 16);
		height: var(--product-row-height);
		box-shadow: calc(var(--shadow-size) / 2) var(--shadow-size) var(--shadow-size) rgb(0 0 0 / 0.5);
		background-color: var(--darkish);
		display: flex;
	}
	.chars {
		height: var(--row-height);
		display: flex;
	}
</style>
