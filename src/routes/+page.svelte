<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	type FileData = {
		name: string;
		modified: number;
		created: number;
	};

	let saves = fetchSaves();
	let saveCreationStatus: Promise<null>;

	let newSaveName = '';

	async function fetchSaves() {
		return (await invoke('fetch_saves')) as FileData[];
	}

	async function createSave(name: string) {
		return (await invoke('create_save', { name })) as null;
	}
</script>

<h1>RIIC Engine</h1>
<p>v0.1.0</p>

{#await saves}
	<p>Loading...</p>
{:then saveList}
	{#if saveList.length > 0}
		{#each saveList.sort((prev, curr) => prev.created - curr.created) as save}
			<p>File name: {save.name}</p>
		{/each}
	{:else}
		<p>No saves found!</p>
	{/if}
	<input type="text" bind:value={newSaveName} />
	{#if newSaveName}
		<button
			on:click={() =>
				(saveCreationStatus = createSave(newSaveName).finally(() => (saves = fetchSaves())))}
		>
			Add new save
		</button>
	{/if}
	{#await saveCreationStatus then _}
		<p>Success!</p>
	{:catch err}
		<p>{err}</p>
	{/await}
{:catch err}
	<p>{err}</p>
{/await}

<style>
	h1,
	p {
		text-align: center;
		color: var(--light);
	}
	h1 {
		font-size: 4em;
	}
</style>
