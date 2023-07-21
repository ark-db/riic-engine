<script lang="ts">
	import { assets } from '$app/paths';
	import items from '$lib/data/items.json';
	import type { Product } from '$lib/types';

	export let product: Product;
	export let size: number;

	let bgWidth: number;
	let bgHeight: number;

	let iconWidth: number;
	let iconHeight: number;

	const { name, rarity } = items[product];
</script>

<div style="--size: {size}px;">
	<img
		src={`${assets}/rarities/${rarity}.webp`}
		alt=""
		width={size}
		height={size}
		bind:naturalWidth={bgWidth}
		bind:naturalHeight={bgHeight}
	/>
	<img
		src={`${assets}/items/${product}.webp`}
		alt={name}
		width={(iconWidth / bgWidth) * size}
		height={(iconHeight / bgHeight) * size}
		bind:naturalWidth={iconWidth}
		bind:naturalHeight={iconHeight}
	/>
</div>

<style>
	div {
		width: var(--size);
		display: grid;
		grid-template: var(--size) / var(--size);
		place-items: center;
	}
	img {
		grid-area: 1 / 1;
	}
</style>
