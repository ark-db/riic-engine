<script lang="ts">
	import { zoomControls } from '$lib/stores';
	import ColumnLines from './ColumnLines.svelte';
	import FacilityGrid from './FacilityGrid.svelte';
	import ZoomControls from './ZoomControls.svelte';

	const { xScale, yScale } = zoomControls;

	const baseColumnGap = 32;
	const baseRowHeight = 72;

	let gridWidth: number;
	let gridHeight: number;

	$: columnGap = $xScale * baseColumnGap;
	$: rowWidth = gridWidth + columnGap + 37;
	$: rowHeight = $yScale * baseRowHeight;
</script>

<ColumnLines {columnGap} bind:gridWidth {gridHeight} />

<FacilityGrid {rowWidth} {rowHeight} bind:gridHeight />

<ZoomControls />
