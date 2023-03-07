<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import plus from '$lib/images/plus.svg';
	import minus from '$lib/images/minus.svg';
	import Slider from './Slider.svelte';

	export let xScale: number;
	export let yScale: number;

	const iconSize = 22;

	const initTweened = () =>
		tweened(1, {
			duration: 400,
			easing: cubicOut
		});

	const x = initTweened();
	const y = initTweened();

	$: xScale = $x;
	$: yScale = $y;
</script>

<div class="zoom-control" style="--zoom-icon-size: {iconSize}px;">
	<div class="x">
		<img src={minus} alt="Zoom out" width={iconSize} height={iconSize} />
		<Slider min={1} max={2} step={0.1} bind:value={$x} />
		<img src={plus} alt="Zoom in" width={iconSize} height={iconSize} />
	</div>
	<div class="y">
		<img class="rotate" src={minus} alt="Zoom out" width={iconSize} height={iconSize} />
		<Slider min={1} max={2} step={0.1} bind:value={$y} />
		<img src={plus} alt="Zoom in" width={iconSize} height={iconSize} />
	</div>
</div>

<style>
	.zoom-control {
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
	.zoom-control > div {
		--blur-effect: blur(2px);
		position: relative;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 16px;
		padding: var(--slider-padding);
		background: rgba(255, 255, 255, 0.04);
		backdrop-filter: var(--blur-effect);
		-webkit-backdrop-filter: var(--blur-effect);
		display: flex;
		align-items: center;
		column-gap: var(--slider-spacing);
	}
	.zoom-control > .x {
		right: var(--shift-distance);
		bottom: calc(0px - 2 * var(--slider-padding));
	}
	.zoom-control > .y {
		rotate: 90deg;
		right: calc(var(--center-size) / 2);
		bottom: var(--shift-distance);
	}
	.zoom-control > .y > .rotate {
		rotate: -90deg;
	}
</style>
