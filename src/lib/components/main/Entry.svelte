<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { FileData } from '@types';
	import { saveSortMode } from '@stores';
	import SaveNameInput from './NameInput.svelte';
	import Modal from '../Modal.svelte';
	export let save: FileData;

	let hovering = false;
	let pendingDelete = false;

	type DispatchEvents = {
		delete: { name: string };
		export: { name: string };
	};
	const dispatch = createEventDispatcher<DispatchEvents>();

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

	function handleExport() {
		dispatch('export', {
			name: save.name
		});
	}

	function handleExportKeydown(event: KeyboardEvent) {
		if (event.key === 'Space' || event.key === 'Enter') handleExport();
	}

	function handleDelete() {
		pendingDelete = false;
		dispatch('delete', {
			name: save.name
		});
	}

	function handleDeleteKeydown(event: KeyboardEvent) {
		if (event.key === 'Space' || event.key === 'Enter') pendingDelete = true;
	}
</script>

<div
	class="container"
	on:mouseenter={() => (hovering = true)}
	on:mouseleave={() => (hovering = false)}
>
	<SaveNameInput text={save.name} />
	<div class="right">
		{#if hovering}
			<div class="settings">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 640 512"
					height="26"
					width="26"
					on:click={handleExport}
					on:keydown={handleExportKeydown}
				>
					<path
						class="export"
						fill="#9a9696"
						d="M32 64C32 28.7 60.7 0 96 0H256V128c0 17.7 14.3 32 32 32H416V288H248c-13.3 0-24 10.7-24 24s10.7 24 24 24H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V64zM416 336V288H526.1l-39-39c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l80 80c9.4 9.4 9.4 24.6 0 33.9l-80 80c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l39-39H416zm0-208H288V0L416 128z"
					/>
				</svg>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 448 512"
					height="20"
					width="20"
					on:click={() => (pendingDelete = true)}
					on:keydown={handleDeleteKeydown}
				>
					<path
						class="trash"
						fill="#9a9696"
						d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
					/>
				</svg>
			</div>
		{:else}
			<p class="time">
				{formatTime(save[$saveSortMode])}
			</p>
		{/if}
	</div>
</div>

{#if pendingDelete}
	<Modal on:close={() => (pendingDelete = false)}>
		<p class="modal-text">Are you sure you want to delete this save file?</p>
		<div class="options">
			<button class="cancel" on:click={() => (pendingDelete = false)}>Cancel</button>
			<button class="delete" on:click={handleDelete}>Delete</button>
		</div>
	</Modal>
{/if}

<style>
	.container {
		border-radius: 0.75em;
		padding: 1em;
		background: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.right {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.settings {
		display: flex;
		align-items: center;
		column-gap: 5px;
	}
	path {
		transition: fill 0.3s;
	}
	path:hover {
		transition: fill 0.15s;
	}
	.export:hover {
		fill: var(--light);
	}
	.trash:hover {
		fill: var(--salmon-strong);
	}
	.time {
		margin: 0;
		padding: 0 0.5em;
		color: var(--gray);
	}
	.modal-text {
		margin-top: 0;
		color: var(--light);
	}
	.options {
		margin-top: 1.5em;
		display: flex;
		column-gap: 1em;
	}
	.options > button {
		border: none;
		border-radius: 0.5em;
		padding: 0.75em;
		font-size: 0.8em;
		font-weight: 600;
		cursor: pointer;
	}
	.cancel {
		background-color: var(--light);
	}
	.delete {
		background-color: var(--salmon-strong);
		color: var(--light);
	}
</style>
