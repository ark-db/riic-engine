<script lang="ts">
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	export let kind: FacilityName;

	const facilityNameToColor: Record<FacilityName, string> = {
		control: 'rgb(35, 80, 80)',
		dormitory: 'rgb(93, 195, 182)',
		hire: 'rgb(147, 147, 147)',
		manufacture: 'rgb(243, 219, 75)',
		meeting: 'rgb(203, 105, 80)',
		power: 'rgb(158, 188, 71)',
		trading: 'rgb(50, 115, 167)',
		training: 'rgb(118, 21, 39)',
		workshop: 'rgb(235, 232, 76)'
	};
</script>

<div class="container" style:border={`0.5em solid ${facilityNameToColor[kind]}`}>
	<p class="facility-name">
		{facilities[kind].name}
	</p>
	<div class="buttons">
		<button>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" height="32" width="32">
				<path
					d="M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z"
					fill="#9a9696"
				/>
			</svg>
		</button>
		<button>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" height="32" width="32">
				<path
					d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
					fill="#9a9696"
				/>
			</svg>
		</button>
	</div>
</div>

<style>
	.container {
		padding: 0.5em 1em;
		display: flex;
		align-items: center;
		column-gap: 1em;
	}
	.facility-name {
		margin: 0;
		color: var(--light);
		font-size: 1.25em;
		font-weight: 600;
	}
	.buttons {
		display: flex;
		align-items: center;
		flex-direction: column;
		row-gap: 0.1em;
	}
	button {
		margin: 0;
		border: none;
		padding: 0;
		background: none;
	}
</style>
