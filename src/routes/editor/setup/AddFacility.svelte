<script lang="ts">
	import facilities from '$lib/data/facilities.json';
	import { tooltip } from '$lib/tooltip';
	import type { FacilityName } from '$lib/types';

	export let kind: FacilityName;
	export let onClick: () => void;

	const { color } = facilities[kind];
</script>

<button
	class="focus-template"
	style="--color: {color};"
	aria-label="Add facility"
	use:tooltip={'Add facility'}
	on:click|trusted={onClick}
>
	<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" width="48" height="48">
		<path
			fill={color}
			d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
		/>
	</svg>
</button>

<style>
	button {
		--focus-border-offset: -8px;
		--border-weight: 5px;
		height: 7em;
		border: var(--border-weight) dashed var(--color);
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
