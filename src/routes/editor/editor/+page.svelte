<script lang="ts">
	import { zoomControls } from '$lib/stores';
	import ColumnLines from './ColumnLines.svelte';
	import FacilityGrid from './FacilityGrid.svelte';
	import ZoomControls from './ZoomControls.svelte';

	const { xScale, yScale } = zoomControls;
</script>

<ColumnLines columnGapScale={$xScale} />

<FacilityGrid rowHeightScale={$yScale} />

<ZoomControls />
