<script>
	import { activeSave } from '$lib/stores';
	import Facility from './Facility.svelte';
</script>

<div class="container">
	<div class="control">
		<Facility kind={'control'} bind:level={$activeSave.data.layout.cc.level} />
	</div>
	<div class="tp-wrapper">
		{#each $activeSave.data.layout.tp as fac}
			<Facility kind={'trading'} bind:level={fac.level} />
		{/each}
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-rows: repeat(5, auto);
		grid-template-columns: repeat(5, 15em);
		gap: 0.5em;
	}
	.control {
		/* grid-area: 'control control'; */
		grid-row: 1 / 2;
		grid-column: 1 / 3;
	}
	.tp-wrapper {
		grid-row: 1 / 6;
		grid-column: 3 / 4;
	}
</style>
