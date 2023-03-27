<script lang="ts">
	import items from '$lib/data/items.json';
	import type { FactoryProduct, TradingProduct } from '$lib/types';
	import ProductIcon from './ProductIcon.svelte';

	export let products: FactoryProduct[] | TradingProduct[];
</script>

<div class:container={products.length > 1}>
	{#each products as product}
		<div class="label">
			<p>{items[product].name}</p>
			<ProductIcon {product} size={64} />
		</div>
	{/each}
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5em;
	}
	.label {
		border: 2px solid transparent;
		border-radius: 8px;
		padding: 0.375em 0.375em 0.375em 0.75em;
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
		column-gap: 1em;
		transition: border 0.2s;
	}
	.label:hover {
		border-color: var(--blue-mild);
	}
	p {
		max-width: 6.5em;
		margin: 0;
		font-size: 1.15em;
		font-weight: 600;
		line-height: 1.4em;
	}
</style>
