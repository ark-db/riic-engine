<script lang="ts">
	import { assets } from '$app/paths';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;
	export let size: number;
</script>

<img src={`${assets}/facilities/${kind}.avif`} alt="Facility icon" width={size} height={size} />
