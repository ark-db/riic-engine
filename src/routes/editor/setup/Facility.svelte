<script lang="ts">
	import { assets } from '$app/paths';
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	import LevelIndicator from './LevelIndicator.svelte';
	export let kind: FacilityName;
	export let level: number;

	let maxLevel = facilities[kind].capacity.length;

	const facilityNameToColor: Record<FacilityName, string> = {
		control: '#005752',
		dormitory: '#21cdcb',
		hire: '#565656',
		manufacture: '#ffd800',
		meeting: '#dd653f',
		power: '#8fc31f',
		trading: '#0075a9',
		training: '#7d0022',
		workshop: '#e3eb00'
	};
</script>

<div class="container" style:border={`0.5em solid ${facilityNameToColor[kind]}`}>
	<div class="info">
		<p class="facility-name">
			{facilities[kind].name}
		</p>
		<div class="graphics">
			<img src={`${assets}/facilities/${kind}.webp`} alt="Facility icon" height="32" width="32" />
			<LevelIndicator {level} --color={facilityNameToColor[kind]} />
		</div>
	</div>

	<div class="buttons">
		{#if level < maxLevel}
			<button class="increment" on:click={() => (level += 1)}>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" height="32" width="32">
					<path
						d="M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z"
						fill="#9a9696"
					/>
				</svg>
			</button>
		{/if}
		{#if level > 1}
			<button class="decrement" on:click={() => (level -= 1)}>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" height="32" width="32">
					<path
						d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
						fill="#9a9696"
					/>
				</svg>
			</button>
		{/if}
	</div>
</div>

<style>
	.container {
		padding: 0.75em;
		display: flex;
		align-items: center;
		column-gap: 1em;
	}
	.info {
		display: flex;
		flex-direction: column;
		row-gap: 0.75em;
	}
	.graphics {
		display: flex;
		align-items: center;
		column-gap: 0.75em;
	}
	.facility-name {
		margin: 0;
		color: var(--light);
		font-size: 1.25em;
		font-weight: 600;
	}
	.buttons {
		position: relative;
	}
	button {
		position: absolute;
		margin: 0.3em 0;
		border: none;
		padding: 0.2em 0.1em;
		background: none;
	}
	.increment {
		bottom: 0;
	}
	.decrement {
		top: 0;
	}
	button:is(:hover, :focus-within) path {
		fill: var(--light);
	}
	path {
		transition: fill 0.3s;
	}
</style>
