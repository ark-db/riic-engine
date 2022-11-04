<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { SaveData } from '@types';
	import { saveSortMode } from '@stores';
	import { tooltip } from '@tooltip';
	import Header from '@main/Header.svelte';
	import Entry from '@main/Entry.svelte';
	import Error from '$lib/components/Error.svelte';
	import addFileIcon from '$lib/images/file-add.svg';
	import refreshIcon from '$lib/images/refresh.svg';

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
	<input
		type="image"
		src={addFileIcon}
		alt="Create new setup"
		width="25"
		height="25"
		title="Create new setup"
		use:tooltip
		on:click={() => (creationState = createSave())}
	/>
	<input
		type="image"
		src={refreshIcon}
		alt="Refresh setup list"
		width="25"
		height="25"
		title="Refresh setup list"
		use:tooltip
		on:click={() => (saves = fetchSaves())}
	/>
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
		column-gap: 5px;
	}
	input {
		border-radius: 5px;
		padding: 7.5px;
		transition: background-color 0.2s;
	}
	input:hover {
		background-color: var(--dark-mild);
		transition: background-color 0.1s;
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
