<script lang="ts">
	import type { SaveData } from '@types';
	import { saveSortMode } from '@stores';
	import SaveNameInput from './NameInput.svelte';
	export let save: SaveData;

	function formatStr(num: number, unit: string): string {
		return `${num} ${unit}${num === 1 ? '' : 's'} ago`;
	}

	function formatTime(secsElapsed: number): string {
		if (secsElapsed < 60) {
			return formatStr(secsElapsed, 'second');
		} else if (secsElapsed < 60 * 60) {
			return formatStr(Math.floor(secsElapsed / 60), 'minute');
		} else if (secsElapsed < 60 * 60 * 24) {
			return formatStr(Math.floor(secsElapsed / (60 * 60)), 'hour');
		} else {
			return formatStr(Math.floor(secsElapsed / (60 * 60 * 24)), 'day');
		}
	}
</script>

<div>
	<SaveNameInput text={save.name} />
	<p class="time">{formatTime(save[$saveSortMode])}</p>
</div>

<style>
	div {
		border-radius: 0.75em;
		padding: 1em;
		background: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	p {
		margin: 0;
		padding: 0 0.5em;
	}
	.time {
		color: var(--gray);
	}
</style>
