<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { SaveData } from '@types';
	import { saveSortMode } from '@stores';
	import Header from '@main/Header.svelte';
	import CreateButton from '@main/CreateButton.svelte';
	import Entry from '@main/Entry.svelte';
	import Error from '$lib/components/Error.svelte';

	let saves = fetchSaves();
	let creationState: Promise<null>;

	async function fetchSaves(): Promise<SaveData[]> {
		return await invoke('fetch_saves');
	}

	async function createSave(): Promise<null> {
		const res = (await invoke('create_save')) as null;
		saves = fetchSaves();
		return res;
	}
</script>

<Header />

<div class="controls">
	<CreateButton on:click={() => (creationState = createSave())} />
</div>

{#await saves}
	<p class="progress-text">Loading saves...</p>
{:then saveList}
	{#if saveList.length > 0}
		<div class="saves">
			{#each saveList.sort((prev, curr) => prev[$saveSortMode] - curr[$saveSortMode]) as save}
				<Entry {save} />
			{/each}
		</div>
	{:else}
		<p class="placeholder">No saves found!</p>
	{/if}
{:catch err}
	<Error msg={err} visible={true} />
{/await}

<!-- prettier-ignore -->
{#await creationState catch err}
	<Error msg={err} visible={true} />
{/await}

<style>
	.controls {
		margin: 1em 0;
		padding: 0 1em;
		display: flex;
		align-items: center;
	}
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
