<script lang="ts">
	import { assets } from '$app/paths';
	import items from '$lib/data/items.json';
	import { tooltip } from '$lib/tooltip';
	import type { Product } from '$lib/types';

	export let item: Product;
	export let size: number;

	let bgWidth: number;
	let bgHeight: number;

	let iconWidth: number;
	let iconHeight: number;

	let { name, rarity } = items[item];
</script>

<div style="--size: {size}px;" use:tooltip={name}>
	<img
		src={`${assets}/rarities/${rarity}.webp`}
		alt=""
		width={size}
		height={size}
		bind:naturalWidth={bgWidth}
		bind:naturalHeight={bgHeight}
	/>
	<img
		src={`${assets}/items/${item}.webp`}
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
