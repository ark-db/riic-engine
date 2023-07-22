<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import hideControls from '$lib/images/ui/zoom-controls-hide.svg';
	import showControls from '$lib/images/ui/zoom-controls-show.svg';
	import { zoomScales } from '$lib/stores';
	import ColumnLines from './ColumnLines.svelte';
	import FacilityGrid from './FacilityGrid.svelte';
	import ZoomControls from './ZoomControls.svelte';

	const { xScale, yScale } = zoomScales;

	const baseColumnWidth = 32;
	const baseRowHeight = 72;

	let gridWidth: number;
	let gridHeight: number;

	$: columnWidth = $xScale * baseColumnWidth;
	$: rowWidth = gridWidth + columnWidth + 39;
	$: rowHeight = $yScale * baseRowHeight;

	let controlsActive = true;

	$: controlsToggleIcon = controlsActive ? hideControls : showControls;
	$: controlsDesc = `${controlsActive ? 'Hide' : 'Show'} zoom controls`;
</script>

<svelte:head>
	<title>RIIC Engine • Editor</title>
</svelte:head>

<div class="main" style="--column-width: {columnWidth}px;">
	<ColumnLines bind:gridWidth {gridHeight} />
	<FacilityGrid bind:gridHeight {rowWidth} {rowHeight} />
</div>

{#if controlsActive}
	<ZoomControls />
{/if}

<div class="controls-toggle">
	<Button desc={controlsDesc} onClick={() => (controlsActive = !controlsActive)}>
		<img src={controlsToggleIcon} alt={controlsDesc} class="menu-icon" width="32" height="32" />
	</Button>
</div>

<style>
	.main {
		/* --facility-edge-padding is used in ./ColumnLines.svelte and ./FacilityRow.svelte */
		--facility-edge-padding: 0.5em;
		/* --shift-box-focus-border-offset is used in ./ColumnLines.svelte, ./FacilityGrid.svelte, and ./ShiftBox.svelte */
		--shift-box-focus-border-offset: -1px;
	}
	.controls-toggle {
		z-index: 1;
		position: fixed;
		bottom: 0;
		right: 0;
		box-shadow: -4px -4px 8px rgb(0 0 0 / 0.2);
		border-width: 1px 0 0 1px;
		border-style: solid;
		border-color: var(--light);
		border-top-left-radius: 8px;
		padding: 4px;
		background-color: black;
	}
</style>
