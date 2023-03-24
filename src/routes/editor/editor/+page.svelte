<script lang="ts">
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
	$: rowWidth = gridWidth + columnWidth + 37;
	$: rowHeight = $yScale * baseRowHeight;
</script>

<svelte:head>
	<title>RIIC Engine • Editor</title>
</svelte:head>

<ColumnLines {columnWidth} bind:gridWidth {gridHeight} />

<FacilityGrid {rowWidth} {rowHeight} bind:gridHeight />

<ZoomControls />
