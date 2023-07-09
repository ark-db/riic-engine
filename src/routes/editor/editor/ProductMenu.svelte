<script lang="ts">
	import items from '$lib/data/items.json';
	import type { TradingProduct, FactoryProduct } from '$lib/types';
	import ProductIcon from './ProductIcon.svelte';

	export let products: TradingProduct[] | FactoryProduct[];
	export let onSelect: (product: (typeof products)[number]) => void;
</script>

<div class:container={products.length > 1}>
	{#each products as product}
		<button class="focus-template product" on:click|trusted={() => onSelect(product)}>
			<p>{items[product].name}</p>
			<ProductIcon {product} size={64} />
		</button>
	{/each}
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5em;
	}
	.product {
		--focus-border-offset: -2px;
		border: 2px solid transparent;
		border-radius: 8px;
		padding: 0.5em 0.5em 0.5em 1em;
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
		column-gap: 1.5em;
		transition: border 0.2s;
	}
	.product:hover {
		border-color: var(--blue-mild);
	}
	p {
		width: 6.5em;
		color: var(--light-strong);
		text-align: left;
		font-size: 1.5em;
		font-weight: 600;
	}
</style>
