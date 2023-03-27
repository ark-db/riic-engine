<script lang="ts">
	import tippy, { type Instance, type Props } from 'tippy.js/headless';

	export let menuTemplate: HTMLDivElement;

	let box: HTMLDivElement;
	let menu: Instance<Props>;

	function render() {
		const popper = document.createElement('div');
		popper.className = menuTemplate.className;
		popper.innerHTML = menuTemplate.innerHTML;
		return { popper };
	}

	function initMenu() {
		if (!menu) {
			menu = tippy(box, {
				arrow: false,
				interactive: true,
				offset: [0, 0],
				placement: 'right-start',
				render,
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
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div>
	<div
		class="focus-template box"
		tabindex="0"
		bind:this={box}
		on:contextmenu|trusted|preventDefault={handleContextOpen}
		on:keydown|trusted={handleKeydown}
	/>
</div>

<style>
	.box {
		--focus-border-offset: -1px;
		--focus-border-radius: 0;
		/* --column-width is defined in ./FacilityRow.svelte */
		--box-width: calc(var(--column-width) + 1em);
		width: var(--box-width);
		/* --row-height is defined in ./FacilityRow.svelte */
		height: var(--row-height);
		position: relative;
	}
</style>
