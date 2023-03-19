<script lang="ts">
	import { activeSave } from '$lib/stores';
	import type { Facility as FacilityType, BoostFacility as BoostFacilityType } from '$lib/types';
	import Facility from './Facility.svelte';
	import AddFacility from './AddFacility.svelte';

	function addFacility(facilities: FacilityType[]) {
		facilities.push({
			level: 1,
			shifts: []
		});
		// notify store subscribers with assignment
		$activeSave = $activeSave;
	}

	function addBoostFacility<P>(facilities: BoostFacilityType<P>[]) {
		facilities.push({
			level: 1,
			shifts: [],
			boosts: [],
			products: []
		});
		// notify store subscribers with assignment
		$activeSave = $activeSave;
	}

	function deleteFacility<P>(facilities: FacilityType[] | BoostFacilityType<P>[], index: number) {
		facilities.splice(index, 1);
		// notify store subscribers with assignment
		$activeSave = $activeSave;
	}
</script>

<svelte:head>
	<title>RIIC Engine • Facility Setup</title>
</svelte:head>

<!-- Note: Some Facility components are contained in {#if true} blocks to make transitions within the component work properly -->
<div class="container">
	<div class="control">
		{#if true}
			<Facility kind="control" minLevel={1} bind:level={$activeSave.data.layout.cc.level} />
		{/if}
	</div>
	<div class="tp-wrapper">
		{#each $activeSave.data.layout.tp as room, i}
			<Facility
				kind="trading"
				minLevel={1}
				onDelete={() => deleteFacility($activeSave.data.layout.tp, i)}
				bind:level={room.level}
			/>
		{/each}
		{#if $activeSave.data.layout.tp.length < 5}
			<AddFacility kind="trading" onClick={() => addBoostFacility($activeSave.data.layout.tp)} />
		{/if}
	</div>
	<div class="fac-wrapper">
		{#each $activeSave.data.layout.fac as room, i}
			<Facility
				kind="manufacture"
				minLevel={1}
				onDelete={() => deleteFacility($activeSave.data.layout.fac, i)}
				bind:level={room.level}
			/>
		{/each}
		{#if $activeSave.data.layout.fac.length < 5}
			<AddFacility
				kind="manufacture"
				onClick={() => addBoostFacility($activeSave.data.layout.fac)}
			/>
		{/if}
	</div>
	<div class="pp-wrapper">
		{#each $activeSave.data.layout.pp as room, i}
			<Facility
				kind="power"
				minLevel={1}
				onDelete={() => deleteFacility($activeSave.data.layout.pp, i)}
				bind:level={room.level}
			/>
		{/each}
		{#if $activeSave.data.layout.pp.length < 5}
			<AddFacility kind="power" onClick={() => addFacility($activeSave.data.layout.pp)} />
		{/if}
	</div>
	<div class="dorm-wrapper">
		{#each $activeSave.data.layout.dorm as room}
			<Facility kind="dormitory" bind:level={room.level} />
		{/each}
	</div>
	<div class="workshop">
		{#if true}
			<Facility kind="workshop" minLevel={1} bind:level={$activeSave.data.layout.workshop.level} />
		{/if}
	</div>
	<div class="reception">
		{#if true}
			<Facility kind="meeting" bind:level={$activeSave.data.layout.rr.level} />
		{/if}
	</div>
	<div class="office">
		{#if true}
			<Facility kind="hire" bind:level={$activeSave.data.layout.office.level} />
		{/if}
	</div>
	<div class="training">
		{#if true}
			<Facility kind="training" bind:level={$activeSave.data.layout.train.level} />
		{/if}
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-rows: repeat(5, auto);
		grid-template-columns: repeat(5, minmax(15em, 24em));
		gap: 0.75em;
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
	.tp-wrapper,
	.fac-wrapper,
	.pp-wrapper,
	.dorm-wrapper {
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
</style>
