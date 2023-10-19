<script lang="ts">
	import facilities from '$lib/data/facilities.json';
	import powerIcon from '$lib/images/ui/power.webp';
	import slash from '$lib/images/ui/slash.webp';
	import { activeSave } from '$lib/stores';
	import { tooltip } from '$lib/tooltip';
	import type { SaveData } from '$lib/types';

	$: powerUsage = calculatePowerUsage($activeSave);
	$: maxPower = calculatePowerCapacity($activeSave);

	// Calculates power usage of all facilities from a base layout
	function calculatePowerUsage({
		layout: { dorm, tp, fac, workshop, rr, office, train }
	}: SaveData): number {
		let power = 0;
		// Control Center power consumption is 0 at all levels, so it is not included
		for (const { level } of dorm) {
			power -= level === 0 ? 0 : facilities.dormitory.power.at(level - 1) ?? 0;
		}
		// Factory, trading post, and workshop level must be at least 1, so checking for level 0 (unbuilt) is unnecessary
		for (const { level } of tp) {
			power -= facilities.trading.power.at(level - 1) ?? 0;
		}
		for (const { level } of fac) {
			power -= facilities.manufacture.power.at(level - 1) ?? 0;
		}
		power -= facilities.workshop.power.at(workshop.level - 1) ?? 0;
		power -= rr.level === 0 ? 0 : facilities.meeting.power.at(rr.level - 1) ?? 0;
		power -= office.level === 0 ? 0 : facilities.hire.power.at(office.level - 1) ?? 0;
		power -= train.level === 0 ? 0 : facilities.training.power.at(train.level - 1) ?? 0;

		return power;
	}

	// Calculates the maximum power available from a base layout
	function calculatePowerCapacity({ layout: { pp } }: SaveData): number {
		return pp.reduce(
			(partialSum, facility) => partialSum + (facilities.power.power.at(facility.level - 1) ?? 0),
			0
		);
	}
</script>

<div class="container" use:tooltip={'Power usage'}>
	<img src={powerIcon} alt="Power usage icon" width="36" height="36" />
	<div class="text" aria-live="polite">
		<p class="stats" class:invalid={powerUsage < 0 || powerUsage > maxPower}>
			{powerUsage}
		</p>
		<img class="divider" src={slash} alt="slash" width="13" height="31" />
		<p class="stats">{maxPower}</p>
	</div>
</div>

<style>
	.container {
		padding: 0 0.5em;
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
		color: var(--light);
		font-size: var(--font-size);
		font-weight: 600;
		&.invalid {
			color: var(--salmon-strong);
		}
	}
	.divider {
		--height: clamp(30px, 3.5vh, 40px);
		width: calc(var(--height) / 2.5);
		height: var(--height);
		margin: 0 0.2em;
	}
</style>
