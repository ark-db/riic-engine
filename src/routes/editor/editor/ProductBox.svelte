<script lang="ts">
	import tippy, { type Instance, type Props } from 'tippy.js/headless';
	import items from '$lib/data/items.json';
	import type { Product } from '$lib/types';
	import ProductMenu from './ProductMenu.svelte';

	export let kind: 'trading' | 'manufacture';
	export let level: number;
	export let product: Product | undefined;
	export let onSetProduct: (product: Product) => void;

	let box: HTMLDivElement;
	let template: HTMLDivElement;
	let menu: Instance<Props>;

	function initMenu() {
		if (!menu) {
			menu = tippy(box, {
				arrow: false,
				interactive: true,
				offset: [0, 12],
				placement: 'auto-start',
				render: () => ({ popper: template }),
				trigger: 'manual'
			});
		}
	}

	function handleContextOpen({ x, y }: MouseEvent) {
		initMenu();
		menu.setProps({
			getReferenceClientRect: () => ({
				width: 0,
				height: 0,
				top: y,
				bottom: y,
				left: x,
				right: x,
				x,
				y,
				toJSON: () => ({ x, y })
			})
		});
		menu.show();
	}

	function handleKeydown({ key }: KeyboardEvent) {
		if (key === 'Enter') {
			initMenu();
			menu.show();
		}
	}

	function handleFocusout({ relatedTarget }: FocusEvent) {
		if (!(relatedTarget instanceof Node && template.contains(relatedTarget))) {
			menu.hide();
		}
	}

	function handleSelect(product: Product) {
		menu.hide();
		onSetProduct(product);
	}
</script>

<!-- This outer div exists so that the context menu created by tippy.js is accessible via keyboard navigation -->
<div>
	<div
		class="focus-template box"
		style="background-color: {product ? items[product].color : ''}"
		tabindex="0"
		role="button"
		bind:this={box}
		on:contextmenu|trusted|preventDefault={handleContextOpen}
		on:keydown|trusted={handleKeydown}
	/>
</div>

<div class="template" hidden>
	<div class="tooltip-template" bind:this={template} on:focusout|trusted={handleFocusout}>
		{#if kind === 'trading'}
			{#if level === 3}
				<ProductMenu products={['lmd', 'orundum']} onSelect={handleSelect} />
			{:else}
				<ProductMenu products={['lmd']} onSelect={handleSelect} />
			{/if}
		{:else if kind === 'manufacture'}
			{#if level === 3}
				<ProductMenu
					products={['exp200', 'exp400', 'exp1000', 'gold', 'shard']}
					onSelect={handleSelect}
				/>
			{:else if level === 2}
				<ProductMenu products={['exp200', 'exp400', 'gold']} onSelect={handleSelect} />
			{:else if level === 1}
				<ProductMenu products={['exp200', 'gold']} onSelect={handleSelect} />
			{/if}
		{/if}
	</div>
</div>

<style>
	.box {
		--focus-border-offset: -1px;
		--focus-border-radius: 0;
		/* --box-width is defined in ./FacilityRow.svelte */
		width: var(--box-width);
		/* --product-row-height is defined in ./FacilityRow.svelte */
		height: var(--product-row-height);
	}
	.template {
		display: none;
	}
</style>
