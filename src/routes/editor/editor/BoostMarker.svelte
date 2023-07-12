<script lang="ts">
	import tippy, { type Instance, type Props } from 'tippy.js/headless';

	// export let onSetDrones: (drones: number) => void;

	let box: HTMLDivElement;
	let template: HTMLDivElement;
	let menu: Instance<Props>;

	let menuActive = false;

	function initMenu() {
		if (!menu) {
			menuActive = true;
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
		if (!(relatedTarget instanceof Node && template.contains(relatedTarget))) menu.hide();
	}
</script>

<div class="container">
	<div
		class="focus-template marker"
		tabindex="0"
		role="button"
		bind:this={box}
		on:contextmenu|trusted|preventDefault={handleContextOpen}
		on:keydown|trusted={handleKeydown}
	/>
</div>

<div class="template" hidden>
	<div class="tooltip-template" bind:this={template} on:focusout={handleFocusout}>
		{#if menuActive}
			<!-- TODO -->
		{/if}
	</div>
</div>

<style>
	.container {
		z-index: 1;
	}
	.marker {
		--focus-border-offset: -4px;
		--focus-border-radius: 4px;
		/* --marker-width is defined in ./FacilityRow.svelte */
		width: var(--marker-width);
		height: 100%;
		background-color: rgb(221 160 221 / 0.4);
	}
	.template {
		display: none;
	}
</style>
