<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { saveList, saveSortMode, saveSortOrder } from '$lib/stores';
	import { tooltip } from '$lib/tooltip';
	import Header from './Header.svelte';
	import Entry from './Entry.svelte';
	import addFileIcon from '$lib/images/file-add.svg';
	import refreshIcon from '$lib/images/refresh.svg';

	invoke<void>('rename_window');

	saveList.load().then(() => invoke<void>('show_window'));
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
			use:tooltip={'Create new setup'}
			on:click={saveList.create}
		/>
		<input
			type="image"
			src={refreshIcon}
			alt="Refresh setup list"
			width="25"
			height="25"
			use:tooltip={'Refresh setup list'}
			on:click={saveList.load}
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
				use:tooltip={saveSortMode.nextDesc()}
				on:click={saveSortMode.toggle}
			/>
		{/key}
		{#key $saveSortOrder}
			<input
				type="image"
				src={saveSortOrder.src()}
				alt={saveSortOrder.nextDesc()}
				width="25"
				height="25"
				use:tooltip={saveSortOrder.nextDesc()}
				on:click={saveSortOrder.toggle}
			/>
		{/key}
	</div>
</div>

{#if $saveList && $saveList.length > 0}
	<main>
		{#each $saveList.sort((prev, curr) => (prev[$saveSortMode] - curr[$saveSortMode]) * ($saveSortOrder === 'increasing' ? 1 : -1)) as save}
			<Entry {save} />
		{/each}
	</main>
{:else}
	<p class="placeholder">No saves found!</p>
{/if}

<style>
	.controls,
	.left,
	.right {
		display: flex;
		align-items: center;
		column-gap: 0.5em;
	}
	.controls {
		margin: 0.75em 0;
		padding: 0 0.75em;
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
	main {
		padding: 0.25em 0.5em 0.5em;
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
