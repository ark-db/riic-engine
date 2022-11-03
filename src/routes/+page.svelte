<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { SaveData } from '@types';
	import { saveSortMode } from '@stores';
	import Header from '$lib/components/Header.svelte';
	import SaveEntry from '$lib/components/SaveEntry.svelte';
	import Error from '$lib/components/Error.svelte';

	let saves = fetchSaves();

	async function fetchSaves(): Promise<SaveData[]> {
		return await invoke('fetch_saves');
	}
</script>

<Header />

{#await saves}
	<p class="progress-text">Loading saves...</p>
{:then saveList}
	{#if saveList.length > 0}
		<div class="saves">
			{#each saveList.sort((prev, curr) => prev[$saveSortMode] - curr[$saveSortMode]) as save}
				<SaveEntry {save} />
			{/each}
		</div>
	{:else}
		<p class="placeholder">No saves found!</p>
	{/if}
{:catch err}
	<Error msg={err} visible={true} />
{/await}

<style>
	.progress-text {
		border-radius: 1em;
		padding: 1em;
		background-color: var(--dark-strong);
		text-align: center;
		color: var(--light-strong);
	}
	.saves {
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
	}
	.placeholder {
		border-radius: 1em;
		padding: 1em;
		background-color: var(--dark-strong);
		text-align: center;
		color: var(--light-strong);
	}
</style>
