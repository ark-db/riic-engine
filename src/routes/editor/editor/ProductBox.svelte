<script lang="ts">
	import items from '$lib/data/items.json';
	import type { Product } from '$lib/types';
	import InteractionBox from './InteractionBox.svelte';
	import ProductMenu from './ProductMenu.svelte';

	export let kind: 'trading' | 'manufacture';
	export let level: number;
	export let product: Product | undefined;
	export let onSetProduct: (product: Product) => void;
</script>

<InteractionBox menuOptions={{ offset: [0, 12], placement: 'auto-start' }}>
	<div
		slot="base"
		class="focus-template box"
		style="background-color: {product ? items[product].color : ''}"
		tabindex="0"
		role="button"
	/>

	<svelte:fragment slot="menu">
		{#if kind === 'trading'}
			{#if level === 3}
				<ProductMenu products={['lmd', 'orundum']} onSelect={onSetProduct} />
			{:else}
				<ProductMenu products={['lmd']} onSelect={onSetProduct} />
			{/if}
		{:else if kind === 'manufacture'}
			{#if level === 3}
				<ProductMenu
					products={['exp200', 'exp400', 'exp1000', 'gold', 'shard']}
					onSelect={onSetProduct}
				/>
			{:else if level === 2}
				<ProductMenu products={['exp200', 'exp400', 'gold']} onSelect={onSetProduct} />
			{:else if level === 1}
				<ProductMenu products={['exp200', 'gold']} onSelect={onSetProduct} />
			{/if}
		{/if}
	</svelte:fragment>
</InteractionBox>

<style>
	.box {
		--focus-border-offset: -2px;
		--focus-border-radius: 2px;
		/* --box-width is defined in ./FacilityRow.svelte */
		width: var(--box-width);
		/* --product-row-height is defined in ./FacilityRow.svelte */
		height: var(--product-row-height);
	}
</style>
