<script lang="ts">
	import ColumnLines from './ColumnLines.svelte';
	import FacilityGrid from './FacilityGrid.svelte';
	import ZoomControls from './ZoomControls.svelte';

	let xScale: number;
	let yScale: number;
</script>

<ColumnLines columnGapScale={xScale} />

<FacilityGrid rowHeightScale={yScale} />

<ZoomControls bind:xScale bind:yScale />
