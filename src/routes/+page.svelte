<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Button from '$lib/components/Button.svelte';
	import addFileIcon from '$lib/images/ui/file-add.svg';
	import refreshIcon from '$lib/images/ui/refresh.svg';
	import { saveList, saveSortMode, saveSortOrder } from '$lib/stores';
	import ExportNotice from './ExportNotice.svelte';
	import Header from './Header.svelte';
	import Entry from './Entry.svelte';

	// Rename the app window when moving from the editor to the main menu
	invoke<void>('rename_window');

	// Upon app startup, load save list before displaying window
	saveList.load().then(() => invoke<void>('show_window'));
</script>

<svelte:head>
	<title>RIIC Engine • Home</title>
</svelte:head>

<ExportNotice />

<Header />

<div class="controls">
	<div class="left">
		<Button desc="Create new setup" onClick={saveList.create}>
			<img src={addFileIcon} alt="Create new setup" width="25" height="25" />
		</Button>
		<Button desc="Refresh setup list" onClick={saveList.load}>
			<img src={refreshIcon} alt="Refresh setup list" width="25" height="25" />
		</Button>
	</div>
	<div class="right">
		<Button desc={$saveSortMode.nextDesc} onClick={saveSortMode.toggle}>
			<img src={saveSortMode.src()} alt={$saveSortMode.nextDesc} width="25" height="25" />
		</Button>
		<Button desc={$saveSortOrder.nextDesc} onClick={saveSortOrder.toggle}>
			<img src={saveSortOrder.src()} alt={$saveSortOrder.nextDesc} width="25" height="25" />
		</Button>
	</div>
</div>

{#if $saveList.length > 0}
	<main>
		<ol>
			{#each $saveList as save}
				<Entry {save} />
			{/each}
		</ol>
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
	img {
		border-radius: 5px;
		padding: 7.5px;
		transition: background-color 0.2s;
		&:hover {
			background-color: var(--dark-mild);
		}
	}
	main {
		padding: 0.25em 0.5em 0.5em;
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
	}
	ol {
		list-style: none;
		padding: 0;
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
	}
	.placeholder {
		margin: 0 1em;
		border-radius: 1em;
		padding: 1em;
		background-color: var(--dark-strong);
		text-align: center;
		color: var(--light-strong);
	}
</style>
