<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { FileData } from '@types';
	import { saveSortMode, saveSortOrder } from '@stores';
	import { tooltip } from '@tooltip';
	import Header from '@main/Header.svelte';
	import Entry from '@main/Entry.svelte';
	import Error from '$lib/components/Error.svelte';
	import addFileIcon from '$lib/images/file-add.svg';
	import refreshIcon from '$lib/images/refresh.svg';

	let saves: Promise<FileData[]>;
	let processState: Promise<void>;

	const refresh = () => (saves = invoke<FileData[]>('fetch_saves'));

	refresh();

	function createSave() {
		processState = invoke<void>('create_save');
		refresh();
	}

	function exportSave(event: CustomEvent<{ name: string }>) {
		processState = invoke<void>('export_save', {
			name: event.detail.name
		});
		refresh();
	}

	function deleteSave(event: CustomEvent<{ name: string }>) {
		processState = invoke<void>('delete_save', {
			name: event.detail.name
		});
		refresh();
	}
</script>

<Header />

<div class="controls">
	<div class="left">
		<input
			type="image"
			src={addFileIcon}
			alt="Create new setup"
			width="25"
			height="25"
			title="Create new setup"
			use:tooltip
			on:click={createSave}
		/>
		<input
			type="image"
			src={refreshIcon}
			alt="Refresh setup list"
			width="25"
			height="25"
			title="Refresh setup list"
			use:tooltip
			on:click={refresh}
		/>
	</div>
	<div class="right">
		{#key $saveSortMode}
			<input
				type="image"
				src={saveSortMode.src()}
				alt={saveSortMode.nextDesc()}
				width="25"
				height="25"
				title={saveSortMode.nextDesc()}
				use:tooltip
				on:click={() => {
					saveSortMode.toggle();
					refresh();
				}}
			/>
		{/key}
		{#key $saveSortOrder}
			<input
				type="image"
				src={saveSortOrder.src()}
				alt={saveSortOrder.nextDesc()}
				width="25"
				height="25"
				title={saveSortOrder.nextDesc()}
				use:tooltip
				on:click={() => {
					saveSortOrder.toggle();
					refresh();
				}}
			/>
		{/key}
	</div>
</div>

{#await saves}
	<p class="progress-text">Loading saves...</p>
{:then saveList}
	{#if saveList.length > 0}
		<div class="saves">
			{#each saveList.sort((prev, curr) => (prev[$saveSortMode] - curr[$saveSortMode]) * ($saveSortOrder === 'increasing' ? 1 : -1)) as save}
				<Entry {save} on:export={exportSave} on:delete={deleteSave} />
			{/each}
		</div>
	{:else}
		<p class="placeholder">No saves found!</p>
	{/if}
{:catch err}
	<Error msg={err} visible={true} />
{/await}

{#await processState catch err}
	<Error msg={err} visible={true} />
{/await}

<style>
	.controls,
	.left,
	.right {
		display: flex;
		align-items: center;
		column-gap: 5px;
	}
	.controls {
		margin: 1em 0;
		padding: 0 1em;
		justify-content: space-between;
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
