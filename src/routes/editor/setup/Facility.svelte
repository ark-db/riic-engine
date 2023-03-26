<script lang="ts">
	import { scale } from 'svelte/transition';
	import FacilityIcon from '$lib/components/FacilityIcon.svelte';
	import Button from '$lib/components/Button.svelte';
	import facilities from '$lib/data/facilities.json';
	import type { FacilityName } from '$lib/types';
	import LevelIndicator from './LevelIndicator.svelte';

	export let kind: FacilityName;
	export let level: number;
	export let minLevel = 0;
	// A specified callback for the `onDelete` prop implies that a facility can be deleted
	export let onDelete: (() => void) | undefined = undefined;

	let facility = facilities[kind];
	let maxLevel = facility.capacity.length;
</script>

<div
	class="container"
	class:unused={level === 0}
	style="--facility-color: {facility.color};"
	in:scale|local
>
	<div class="info">
		<p class="facility-name">
			{facility.name}
		</p>
		<div class="graphics">
			<FacilityIcon {kind} size={36} />
			<LevelIndicator {level} />
		</div>
	</div>

	<div class="buttons">
		{#if level < maxLevel}
			<div class="increment">
				<Button desc="Upgrade level" onClick={() => (level += 1)}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="32" height="32">
						<path
							d="M233.4 105.4c12.5-12.5 32.8-12.5 45.3 0l192 192c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L256 173.3 86.6 342.6c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3l192-192z"
							fill="#9a9696"
						/>
					</svg>
				</Button>
			</div>
		{/if}
		{#if level > minLevel}
			<div class="decrement">
				<Button desc="Downgrade level" onClick={() => (level -= 1)}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="32" height="32">
						<path
							d="M233.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L256 338.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"
							fill="#9a9696"
						/>
					</svg>
				</Button>
			</div>
		{:else if onDelete}
			<div class="delete">
				<Button desc="Remove facility" onClick={onDelete}>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" width="28" height="28">
						<path
							d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
							fill="#9a9696"
						/>
					</svg>
				</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	.container {
		/* --facility-color is used in ./LevelIndicator.svelte */
		border: 0.5em solid var(--facility-color);
		padding: 0.75em;
		background: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
		transition: filter 0.3s;
		/* translate3d() prevents elements from flickering after transition on WebKit */
		transform: translate3d(0, 0, 0);
	}
	.container.unused {
		filter: brightness(0.6);
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
	.buttons > div {
		--edge-gap: -0.15em;
		position: absolute;
		margin: 0.2em 0;
		border: none;
		padding: 0.25em;
		background: none;
	}
	.increment {
		bottom: var(--edge-gap);
	}
	.decrement,
	.delete {
		top: var(--edge-gap);
	}
	.increment:is(:hover, :focus-within) path,
	.decrement:is(:hover, :focus-within) path {
		fill: var(--light);
	}
	.delete:is(:hover, :focus-within) path {
		fill: var(--salmon-strong);
	}
	path {
		transition: fill 0.3s;
	}
</style>
