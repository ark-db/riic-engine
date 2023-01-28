<script lang="ts">
	import { assets } from '$app/paths';
	import type { FacilityName } from '$lib/types';
	export let kind: FacilityName;
	export let size: number;
</script>

<img src={`${assets}/facilities/${kind}.webp`} alt="Facility icon" height={size} width={size} />
