<script lang="ts">
	import facilityData from '$lib/data/facilities.json';
	import powerIcon from '$lib/images/power.webp';
	import slash from '$lib/images/slash.webp';
	import { activeSave } from '$lib/stores';
	import type { ActiveSave } from '$lib/types';

	$: powerUsage = calculatePowerUsage($activeSave);
	$: maxPower = calculatePowerCapacity($activeSave);

	// Calculates power usage of all facilities from a base layout
	function calculatePowerUsage(save: ActiveSave): number {
		let power = 0;
		// Control Center power consumption is 0 at all levels,
		// so it is not part of the calculation.
		for (const fac of save.data.layout.dorm) {
			power -= fac.level === 0 ? 0 : facilityData.dormitory.power.at(fac.level - 1) ?? 0;
		}
		// factory, trading post, and workshop level must be at least 1, so checking
		// for level 0 (uninitialized) is unnecessary
		for (const fac of save.data.layout.tp) {
			power -= facilityData.trading.power.at(fac.level - 1) ?? 0;
		}
		for (const fac of save.data.layout.fac) {
			power -= facilityData.manufacture.power.at(fac.level - 1) ?? 0;
		}
		power -= facilityData.workshop.power.at(save.data.layout.workshop.level - 1) ?? 0;
		power -=
			save.data.layout.rr.level === 0
				? 0
				: facilityData.meeting.power.at(save.data.layout.rr.level - 1) ?? 0;
		power -=
			save.data.layout.office.level === 0
				? 0
				: facilityData.hire.power.at(save.data.layout.office.level - 1) ?? 0;
		power -=
			save.data.layout.train.level === 0
				? 0
				: facilityData.training.power.at(save.data.layout.train.level - 1) ?? 0;

		return power;
	}

	// Calculates the maximum power available from a base layout
	function calculatePowerCapacity(save: ActiveSave): number {
		return save.data.layout.pp.reduce(
			(partialSum, facility) => partialSum + (facilityData.power.power.at(facility.level - 1) ?? 0),
			0
		);
	}
</script>

<div class="container">
	<img src={powerIcon} alt="Power usage icon" width="34" height="34" />
	<div class="text">
		<p class="stats">
			<span class:invalid={powerUsage < 0 || powerUsage > maxPower}>
				{powerUsage}
			</span>
		</p>
		<img class="divider" src={slash} alt="slash" width="13" height="31" />
		<p class="stats">{maxPower}</p>
	</div>
</div>

<style>
	.container {
		padding: 0 0.5em;
		color: var(--light);
		display: flex;
		align-items: center;
		justify-content: space-between;
		column-gap: 0.5em;
	}
	.text {
		display: flex;
		align-items: center;
	}
	.stats {
		margin: 0;
		font-size: clamp(1.25em, 2.5vh, 1.5em);
		font-weight: 600;
	}
	.invalid {
		color: var(--salmon-strong);
	}
	.divider {
		margin: 0 0.2em;
		--height: clamp(30px, 3.5vh, 40px);
		height: var(--height);
		width: calc(var(--height) / 2.5);
	}
</style>