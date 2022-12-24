<script>
	import { activeSave } from '$lib/stores';
	import Facility from './Facility.svelte';
</script>

<Facility kind={'control'} bind:level={$activeSave.data.layout.cc.level} />
