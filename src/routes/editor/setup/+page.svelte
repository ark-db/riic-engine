<script>
	import { activeSave } from '$lib/stores';
	import Facility from './Facility.svelte';
</script>

<div class="container">
	<div class="control">
		<Facility kind={'control'} bind:level={$activeSave.data.layout.cc.level} />
	</div>
	<div class="tp-wrapper">
		{#each $activeSave.data.layout.tp as facility}
			<Facility kind={'trading'} bind:level={facility.level} />
		{/each}
	</div>
	<div class="fac-wrapper">
		{#each $activeSave.data.layout.fac as facility}
			<Facility kind={'manufacture'} bind:level={facility.level} />
		{/each}
	</div>
	<div class="pp-wrapper">
		{#each $activeSave.data.layout.pp as facility}
			<Facility kind={'power'} bind:level={facility.level} />
		{/each}
	</div>
	<div class="dorm-wrapper">
		{#each $activeSave.data.layout.dorm as facility}
			<Facility kind={'dormitory'} bind:level={facility.level} />
		{/each}
	</div>
	<div class="workshop">
		<Facility kind={'workshop'} bind:level={$activeSave.data.layout.workshop.level} />
	</div>
	<div class="reception">
		<Facility kind={'meeting'} bind:level={$activeSave.data.layout.rr.level} />
	</div>
	<div class="office">
		<Facility kind={'hire'} bind:level={$activeSave.data.layout.office.level} />
	</div>
	<div class="training">
		<Facility kind={'training'} bind:level={$activeSave.data.layout.train.level} />
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
		grid-row: 1 / 2;
		grid-column: 1 / 3;
	}
	.tp-wrapper {
		grid-row: 1 / 6;
		grid-column: 3 / 4;
	}
	.fac-wrapper {
		grid-row: 1 / 6;
		grid-column: 4 / 5;
	}
	.pp-wrapper {
		grid-row: 1 / 6;
		grid-column: 5 / 6;
	}
	.dorm-wrapper {
		grid-row: 2 / 6;
		grid-column: 1 / 2;
	}
	.workshop {
		grid-row: 2 / 3;
		grid-column: 2 / 3;
	}
	.reception {
		grid-row: 3 / 4;
		grid-column: 2 / 3;
	}
	.office {
		grid-row: 4 / 5;
		grid-column: 2 / 3;
	}
	.training {
		grid-row: 5 / 6;
		grid-column: 2 / 3;
	}
</style>
