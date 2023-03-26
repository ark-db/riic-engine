<script lang="ts">
	// import { draggable } from '@neodrag/svelte';
	import { context } from '$lib/ctxmenu';

	export let columnWidth: number;

	const contextMenu = `
		<div class="product-ctxmenu">
			<p>todo</p>
			<p>todo</p>
		</div>
	`;
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- This outer div exists so that the context menu created by tippy.js is accessible via keyboard navigation -->
<div>
	<div
		class="focus-template box"
		style="--column-width: {columnWidth}px;"
		tabindex="0"
		use:context={contextMenu}
	>
		<div class="handle left" />
		<div class="handle right" />
	</div>
</div>

<style>
	.box {
		--focus-border-offset: -1px;
		--focus-border-radius: 0;
		--box-width: calc(var(--column-width) + 1em);
		width: var(--box-width);
		/* --row-height is defined in ./FacilityRow.svelte */
		height: var(--row-height);
		position: relative;
	}
	.handle {
		width: calc(var(--box-width) / 8);
		height: var(--row-height);
		position: absolute;
	}
	.left {
		left: 0;
		cursor: w-resize;
	}
	.right {
		right: 0;
		cursor: e-resize;
	}
</style>
