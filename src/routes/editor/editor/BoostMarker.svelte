<script lang="ts">
	import tippy, { type Instance, type Props } from 'tippy.js/headless';
	import NumberInput from '$lib/components/NumberInput.svelte';
	import droneIcon from '$lib/images/ui/drones.webp';

	export let drones: number | undefined;
	export let onSetDrones: (drones: number) => void;

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
		class:active={drones && drones > 0}
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
			<NumberInput
				desc="Drone count"
				min={0}
				max={999999}
				initial={drones ?? 0}
				placeholder="Drones"
				onValidInput={onSetDrones}
				errorMsg="The specified drone count should be a number from 0 to 999999"
				iconSrc={droneIcon}
				iconSize={28}
			/>
		{/if}
	</div>
</div>

<style>
	.marker {
		--focus-border-offset: -4px;
		--focus-border-radius: 4px;
		z-index: 1;
		/* --marker-width is defined in ./FacilityRow.svelte */
		width: var(--marker-width);
		height: 100%;
		background-color: rgb(221 160 221 / 0.4);
	}
	.marker.active {
		background-color: rgb(221 160 221 / 0.7);
	}
	.template {
		display: none;
	}
	.tooltip-template {
		padding: 0.75em 0.25em 0.75em 0.125em;
	}
</style>
