<script lang="ts">
	import Drones from './Drones.svelte';
	import PowerUsage from './PowerUsage.svelte';
</script>

<div>
	<Drones />
	<PowerUsage />
</div>

<style>
	div {
		--font-size: clamp(1.25em, 2.5vh, 1.5em);
		display: flex;
		align-items: center;
	}
</style>
