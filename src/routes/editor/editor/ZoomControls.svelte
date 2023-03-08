<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import Button from '$lib/components/Button.svelte';
	import plus from '$lib/images/plus.svg';
	import minus from '$lib/images/minus.svg';
	import Slider from './Slider.svelte';

	export let xScale: number;
	export let yScale: number;

	const iconSize = 22;
	const minZoom = 1;
	const maxZoom = 2;
	const zoomStep = 0.125;

	const initTweened = () =>
		tweened(1, {
			duration: 300,
			easing: cubicOut
		});

	const x = initTweened();
	const y = initTweened();

	$: xScale = $x;
	$: yScale = $y;

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
	<div class="x">
		<Button desc="Zoom out" onClick={decrementX}>
			<img src={minus} alt="Zoom out" width={iconSize} height={iconSize} />
		</Button>
		<Slider min={minZoom} max={maxZoom} step={0.002} bind:value={$x} />
		<Button desc="Zoom in" onClick={incrementX}>
			<img src={plus} alt="Zoom in" width={iconSize} height={iconSize} />
		</Button>
	</div>
	<div class="y">
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
		--right-shift: 40px;
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
		--blur-effect: blur(2px);
		position: relative;
		box-shadow: 0 4px 12px rgba(0 0 0 / 0.2);
		border: 1px solid rgba(255 255 255 / 0.25);
		border-radius: 16px;
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
