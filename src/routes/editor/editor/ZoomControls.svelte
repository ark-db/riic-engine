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

<div class="controls">
	<div class="center">
		<Button desc={centerDesc} onClick={centerFn}>
			<img src={centerSrc} alt={centerDesc} width="36" height="36" />
		</Button>
	</div>
	<div class="slider x">
		<Button desc="Zoom out" onClick={decrementX}>
			<img src={minus} alt="Zoom out" width="22" height="22" />
		</Button>
		<Slider min={minZoom} max={maxZoom} step={0.002} bind:value={$x} />
		<Button desc="Zoom in" onClick={incrementX}>
			<img src={plus} alt="Zoom in" width="22" height="22" />
		</Button>
	</div>
	<div class="slider y">
		<Button desc="Zoom out" onClick={decrementY}>
			<img src={minus} alt="Zoom out" width="22" height="22" />
		</Button>
		<Slider min={minZoom} max={maxZoom} step={0.002} bind:value={$y} />
		<Button desc="Zoom in" onClick={incrementY}>
			<img src={plus} alt="Zoom in" width="22" height="22" />
		</Button>
	</div>
</div>

<style>
	.controls {
		--center-padding: 0.75em;
		--slider-padding: 1.5em;
		--slider-spacing: 0.75em;
		z-index: 2;
		position: fixed;
		right: 80px;
		bottom: 24px;
	}
	.controls > div {
		--blur-effect: blur(2px);
		position: absolute;
		box-shadow: 0 4px 12px rgba(0 0 0 / 0.2);
		border: 1px solid rgba(255 255 255 / 0.25);
		border-radius: 16px;
		background: rgba(255 255 255 / 0.04);
		backdrop-filter: var(--blur-effect);
		-webkit-backdrop-filter: var(--blur-effect);
	}
	.controls > .center {
		right: 0;
		bottom: 0;
		padding: var(--center-padding);
	}
	.controls > .slider {
		padding: var(--slider-padding);
		display: flex;
		align-items: center;
		column-gap: var(--slider-spacing);
	}
	.controls > .x {
		right: 0;
		bottom: 0;
	}
	.controls > .y {
		rotate: 90deg;
		transform-origin: right;
		right: 0;
		bottom: 0;
	}
	.controls > .y img {
		rotate: -90deg;
	}
</style>
