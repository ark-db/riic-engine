<script lang="ts">
	import { assets } from '$app/paths';
	import { createEventDispatcher } from 'svelte';
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	import LevelIndicator from './LevelIndicator.svelte';
	export let kind: FacilityName;
	export let level: number;
	export let minLevel = 0;
	export let deletable = false;

	let maxLevel = facilities[kind].capacity.length;

	const dispatch = createEventDispatcher<{ delete: Record<string, never> }>();
</script>

<div class="container" style:border={`0.5em solid ${facilities[kind].color}`}>
	<div class="info">
		<p class="facility-name">
			{facilities[kind].name}
		</p>
		<div class="graphics">
			<img src={`${assets}/facilities/${kind}.webp`} alt="Facility icon" height="36" width="36" />
			<LevelIndicator {level} --color={facilities[kind].color} />
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
		{#if level > minLevel}
			<button class="decrement" on:click={() => (level -= 1)}>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" height="32" width="32">
					<path
						d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
						fill="#9a9696"
					/>
				</svg>
			</button>
		{:else if deletable}
			<button class="delete" on:click={() => dispatch('delete')}>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" height="28" width="28">
					<path
						d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
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
		justify-content: space-between;
	}
	.info {
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
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
		right: 1em;
		display: flex;
		justify-content: center;
	}
	button {
		position: absolute;
		margin: 0.2em 0;
		border: none;
		padding: 0.1em;
		background: none;
	}
	.increment {
		bottom: 0;
	}
	.decrement,
	.delete {
		top: 0;
	}
	button:is(:hover, :focus-within) path {
		fill: var(--light);
	}
	.delete:is(:hover, :focus-within) path {
		fill: var(--salmon-strong);
	}
	path {
		transition: fill 0.3s;
	}
</style>
