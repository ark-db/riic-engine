<script lang="ts">
	import ColumnLines from './ColumnLines.svelte';
	import FacilityGrid from './FacilityGrid.svelte';
	import ZoomControls from './ZoomControls.svelte';

	let xScale: number;
	let yScale: number;

	const columnGap = 36;
	let totalColumnWidth: number;
	$: facilityRowWidth = totalColumnWidth + columnGap + 36;
</script>

<ColumnLines {columnGap} bind:totalColumnWidth />

<FacilityGrid rowLength={facilityRowWidth} rowHeightScale={yScale} />

<ZoomControls bind:xScale bind:yScale />
