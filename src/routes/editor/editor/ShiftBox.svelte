<script lang="ts">
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	import InteractionBox from './InteractionBox.svelte';

	export let kind: FacilityName;
	export let level: number;

	const capacity = facilities[kind].capacity[level - 1];
</script>

<InteractionBox>
	<div slot="base" class="focus-template box" tabindex="0" role="button">
		{capacity}
	</div>
	<svelte:fragment slot="menu" />
</InteractionBox>

<style>
	.box {
		--focus-border-offset: -1px;
		--focus-border-radius: 0;
		/* --box-width is defined in ./FacilityRow.svelte */
		width: var(--box-width);
		/* --row-height is defined in ./FacilityRow.svelte */
		height: var(--row-height);
	}
</style>
