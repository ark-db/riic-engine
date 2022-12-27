<script lang="ts">
	import { activeSave } from '$lib/stores';
	import type { Facility as FacilityType } from '$lib/types';
	import Facility from './Facility.svelte';
	import AddFacility from './AddFacility.svelte';

	function addFacility(facilities: FacilityType[]) {
		facilities.push({
			shifts: [],
			level: 1
		});
		$activeSave = $activeSave;
	}

	function deleteFacility(facilities: FacilityType[], index: number) {
		facilities.splice(index, 1);
		$activeSave = $activeSave;
	}
</script>

<div class="container">
	<div class="control">
		<Facility kind={'control'} minLevel={1} bind:level={$activeSave.data.layout.cc.level} />
	</div>
	<div class="tp-wrapper">
		{#each $activeSave.data.layout.tp as tradingPost, i}
			<Facility
				kind={'trading'}
				minLevel={1}
				deletable
				bind:level={tradingPost.level}
				on:delete={() => deleteFacility($activeSave.data.layout.tp, i)}
			/>
		{/each}
		{#if $activeSave.data.layout.tp.length < 5}
			<AddFacility kind={'trading'} on:click={() => addFacility($activeSave.data.layout.tp)} />
		{/if}
	</div>
	<div class="fac-wrapper">
		{#each $activeSave.data.layout.fac as factory, i}
			<Facility
				kind={'manufacture'}
				minLevel={1}
				deletable
				bind:level={factory.level}
				on:delete={() => deleteFacility($activeSave.data.layout.fac, i)}
			/>
		{/each}
		{#if $activeSave.data.layout.fac.length < 5}
			<AddFacility kind={'manufacture'} on:click={() => addFacility($activeSave.data.layout.fac)} />
		{/if}
	</div>
	<div class="pp-wrapper">
		{#each $activeSave.data.layout.pp as powerPlant, i}
			<Facility
				kind={'power'}
				minLevel={1}
				deletable
				bind:level={powerPlant.level}
				on:delete={() => deleteFacility($activeSave.data.layout.pp, i)}
			/>
		{/each}
		{#if $activeSave.data.layout.pp.length < 5}
			<AddFacility kind={'power'} on:click={() => addFacility($activeSave.data.layout.pp)} />
		{/if}
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
	.tp-wrapper,
	.fac-wrapper,
	.pp-wrapper,
	.dorm-wrapper {
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
	}
</style>
