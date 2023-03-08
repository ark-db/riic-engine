<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import Button from '$lib/components/Button.svelte';
	import maximize from '$lib/images/maximize.svg';
	import minimize from '$lib/images/minimize.svg';
	import plus from '$lib/images/plus.svg';
	import minus from '$lib/images/minus.svg';
	import Slider from './Slider.svelte';

	export let xScale: number;
	export let yScale: number;

	const iconSize = 22;
	const minZoom = 1;
	const maxZoom = 4;
	const zoomStep = 0.25;

	const initTweened = () =>
		tweened(1, {
			duration: 300,
			easing: cubicOut
		});

	const x = initTweened();
	const y = initTweened();

	$: xScale = $x;
	$: yScale = $y;

	function setMaxZoom() {
		$x = maxZoom;
		$y = maxZoom;
	}
	function setMinZoom() {
		$x = minZoom;
		$y = minZoom;
	}

	type ZoomMode = 'max' | 'min';
	let centerZoomMode: ZoomMode = 'max';
	$: if ($x === minZoom && $y === minZoom) centerZoomMode = 'max';
	$: if ($x === maxZoom && $y === maxZoom) centerZoomMode = 'min';
	$: centerDesc = centerZoomMode === 'max' ? 'Zoom to maximum' : 'Zoom to minimum';
	$: centerSrc = centerZoomMode === 'max' ? maximize : minimize;
	$: centerFn = centerZoomMode === 'max' ? setMaxZoom : setMinZoom;

	function incrementX() {
		if ($x < maxZoom) $x = Math.min($x + zoomStep, maxZoom);
	}
	function decrementX() {
		if ($x > minZoom) $x = Math.max($x - zoomStep, minZoom);
	}
	function incrementY() {
		if ($y < maxZoom) $y = Math.min($y + zoomStep, maxZoom);
	}
	function decrementY() {
		if ($y > minZoom) $y = Math.max($y - zoomStep, minZoom);
	}
</script>

<div class="controls" style="--zoom-icon-size: {iconSize}px;">
	<div class="center">
		<Button desc={centerDesc} onClick={centerFn}>
			<img src={centerSrc} alt={centerDesc} width="36" height="36" />
		</Button>
	</div>
	<div class="slider x">
		<Button desc="Zoom out" onClick={decrementX}>
			<img src={minus} alt="Zoom out" width={iconSize} height={iconSize} />
		</Button>
		<Slider min={minZoom} max={maxZoom} step={0.002} bind:value={$x} />
		<Button desc="Zoom in" onClick={incrementX}>
			<img src={plus} alt="Zoom in" width={iconSize} height={iconSize} />
		</Button>
	</div>
	<div class="slider y">
		<Button desc="Zoom out" onClick={decrementY}>
			<img src={minus} alt="Zoom out" width={iconSize} height={iconSize} />
		</Button>
		<Slider min={minZoom} max={maxZoom} step={0.002} bind:value={$y} />
		<Button desc="Zoom in" onClick={incrementY}>
			<img src={plus} alt="Zoom in" width={iconSize} height={iconSize} />
		</Button>
	</div>
</div>

<style>
	.controls {
		--right-shift: 48px;
		--slider-length: 15em;
		--slider-padding: 1.5em;
		--slider-spacing: 0.75em;
		--center-size: 32px;
		--center-margin: 16px;
		--shift-distance: calc(
			var(--slider-length) / 2 + var(--slider-spacing) + var(--zoom-icon-size) * 2 +
				var(--center-size) + var(--center-margin)
		);
		z-index: 2;
		position: fixed;
		bottom: 0;
		right: calc(0px - var(--right-shift) - var(--slider-padding));
	}
	.controls > div {
		position: relative;
		box-shadow: 0 4px 12px rgba(0 0 0 / 0.2);
		border: 1px solid rgba(255 255 255 / 0.25);
		border-radius: 16px;
	}
	.controls > .center {
		--icon-width: 36px;
		--center-padding: 0.75em;
		--center-shift: calc(0px - var(--icon-width) - var(--slider-padding) * 2);
		width: var(--icon-width);
		right: calc(var(--center-shift) - 0.75em + var(--center-padding) - var(--slider-spacing));
		bottom: calc(
			var(--center-shift) - 1.375em - var(--center-padding) / 2 - var(--slider-spacing) / 2
		);
		padding: var(--center-padding);
		background-color: var(--dark);
	}
	.controls > .slider {
		--blur-effect: blur(2px);
		padding: var(--slider-padding);
		background: rgba(255 255 255 / 0.04);
		backdrop-filter: var(--blur-effect);
		-webkit-backdrop-filter: var(--blur-effect);
		display: flex;
		align-items: center;
		column-gap: var(--slider-spacing);
	}
	.controls > .x {
		right: var(--shift-distance);
		bottom: calc(0px - 2 * var(--slider-padding));
	}
	.controls > .y {
		rotate: 90deg;
		right: calc(var(--center-size) / 2);
		bottom: var(--shift-distance);
	}
	.controls > .y img {
		rotate: -90deg;
	}
</style>
