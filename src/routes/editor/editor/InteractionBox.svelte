<script lang="ts">
	import tippy, { type Instance, type Props } from 'tippy.js/headless';

	export let active = false;
	export let menuOptions: Partial<Props> = {};

	let box: HTMLDivElement;
	let template: HTMLDivElement;
	let menu: Instance<Props>;

	function initMenu() {
		active = true;
		if (!menu) {
			menu = tippy(box, {
				arrow: false,
				interactive: true,
				render: () => ({ popper: template }),
				trigger: 'manual',
				...menuOptions
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

	// Hide tooltip after it loses focus via tab navigation
	function handleFocusout({ relatedTarget }: FocusEvent) {
		if (!(relatedTarget instanceof Node && template.contains(relatedTarget))) {
			menu.hide();
			active = false;
		}
	}
</script>

<!-- This outer div exists so that the context menu created by tippy.js is accessible via keyboard navigation -->
<div>
	<div
		tabindex="-1"
		role="button"
		bind:this={box}
		on:contextmenu|trusted|preventDefault={handleContextOpen}
		on:keydown|trusted={handleKeydown}
	>
		<slot name="base" />
	</div>
</div>

<div class="template" hidden>
	<div class="tooltip-template" bind:this={template} on:focusout|trusted={handleFocusout}>
		{#if active}
			<slot name="menu" />
		{/if}
	</div>
</div>

<style>
	.template {
		display: none;
	}
</style>
