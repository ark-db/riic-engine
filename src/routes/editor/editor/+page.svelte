<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import plus from '$lib/images/plus.svg';
	import minus from '$lib/images/minus.svg';
	import { activeSave } from '$lib/stores';
	import FacilityRow from './FacilityRow.svelte';
	import ColumnLines from './ColumnLines.svelte';
	import Slider from './Slider.svelte';

	const zoomIconSize = 22;

	const initTweened = () =>
		tweened(1, {
			duration: 400,
			easing: cubicOut
		});

	const xScale = initTweened();
	const yScale = initTweened();

	const columnGap = 36;
	let totalColumnWidth: number;
	$: facilityRowWidth = totalColumnWidth + columnGap + 36;
	let columnHeight: number;
</script>

<ColumnLines {columnGap} {columnHeight} bind:totalColumnWidth />

<div class="facilities" style="--width: {facilityRowWidth}px;" bind:clientHeight={columnHeight}>
	<FacilityRow kind="control" />

	{#each $activeSave.data.layout.tp as _}
		<FacilityRow kind="trading" />
	{/each}

	{#each $activeSave.data.layout.fac as _}
		<FacilityRow kind="manufacture" />
	{/each}

	{#each $activeSave.data.layout.pp as _}
		<FacilityRow kind="power" />
	{/each}

	{#if $activeSave.data.layout.rr.level > 0}
		<FacilityRow kind="meeting" />
	{/if}

	{#if $activeSave.data.layout.office.level > 0}
		<FacilityRow kind="hire" />
	{/if}

	{#each $activeSave.data.layout.dorm as room}
		{#if room.level > 0}
			<FacilityRow kind="dormitory" />
		{/if}
	{/each}
</div>

<div class="zoom-control" style="--zoom-icon-size: {zoomIconSize}px;">
	<div class="x">
		<img src={minus} alt="Zoom out" width={zoomIconSize} height={zoomIconSize} />
		<Slider min={1} max={2} step={0.1} bind:value={$xScale} />
		<img src={plus} alt="Zoom in" width={zoomIconSize} height={zoomIconSize} />
	</div>
	<div class="y">
		<img class="rotate" src={minus} alt="Zoom out" width={zoomIconSize} height={zoomIconSize} />
		<Slider min={1} max={2} step={0.1} bind:value={$yScale} />
		<img src={plus} alt="Zoom in" width={zoomIconSize} height={zoomIconSize} />
	</div>
</div>

<style>
	.facilities {
		width: var(--width);
		margin-top: 0.75em;
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
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
