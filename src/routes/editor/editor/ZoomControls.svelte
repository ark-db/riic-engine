<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import plus from '$lib/images/plus.svg';
	import minus from '$lib/images/minus.svg';
	import { zoomControls, zoomShortcut } from '$lib/stores';
	import Slider from './Slider.svelte';

	export let xScale: number;
	export let yScale: number;

	const { x, y, min, max, changeX, changeY } = zoomControls;
	const zoomStep = 0.25;

	$: xScale = $x;
	$: yScale = $y;
</script>

<div class="controls">
	<div class="center">
		<Button desc={$zoomShortcut.desc} onClick={$zoomShortcut.run}>
			<img src={$zoomShortcut.src} alt={$zoomShortcut.desc} width="36" height="36" />
		</Button>
	</div>
	<div class="slider x">
		<Button desc="Zoom out" onClick={() => changeX(-zoomStep)}>
			<img src={minus} alt="Zoom out" width="22" height="22" />
		</Button>
		<Slider {min} {max} step={0.002} bind:value={$x} />
		<Button desc="Zoom in" onClick={() => changeX(zoomStep)}>
			<img src={plus} alt="Zoom in" width="22" height="22" />
		</Button>
	</div>
	<div class="slider y">
		<Button desc="Zoom out" onClick={() => changeY(-zoomStep)}>
			<img src={minus} alt="Zoom out" width="22" height="22" />
		</Button>
		<Slider {min} {max} step={0.002} bind:value={$y} />
		<Button desc="Zoom in" onClick={() => changeY(zoomStep)}>
			<img src={plus} alt="Zoom in" width="22" height="22" />
		</Button>
	</div>
</div>

<style>
	.controls {
		--window-edge-gap: 24px;
		z-index: 2;
		position: fixed;
		right: calc(var(--window-edge-gap) + 16px);
		bottom: var(--window-edge-gap);
	}
	.controls > div {
		--slider-center-gap: 1em;
		/* slider thumb height = 2em */
		--slider-width: calc(2em + var(--slider-padding) * 2);
		/* center icon size = 36px */
		--center-size: calc(36px + var(--center-padding) * 2);
		--center-margin: calc((var(--slider-width) - var(--center-size)) / 4);
		--center-padding: 0.75em;
		--slider-padding: 1.5em;
		--slider-spacing: 0.75em;
		--blur-effect: blur(5px);
		position: absolute;
		box-shadow: 0 4px 12px rgba(0 0 0 / 0.2);
		border: 1px solid rgba(255 255 255 / 0.25);
		border-radius: 16px;
		background: rgba(255 255 255 / 0.04);
		backdrop-filter: var(--blur-effect);
		-webkit-backdrop-filter: var(--blur-effect);
	}
	.center {
		right: var(--center-margin);
		bottom: var(--center-margin);
		padding: var(--center-padding);
	}
	.slider {
		padding: var(--slider-padding);
		display: flex;
		align-items: center;
		column-gap: var(--slider-spacing);
	}
	.x {
		right: calc(var(--center-size) + var(--center-margin) * 2 + var(--slider-center-gap));
		bottom: 0;
	}
	.y {
		rotate: 90deg;
		transform-origin: right;
		right: 36px;
		bottom: calc(var(--center-size) / 2 + var(--center-margin) + var(--slider-center-gap));
	}
	.y img {
		rotate: -90deg;
	}
</style>
