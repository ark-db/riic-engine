<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { FileData } from '@types';
	import { saveSortMode } from '@stores';
	import SaveNameInput from './NameInput.svelte';
	import Modal from '../Modal.svelte';
	export let save: FileData;

	let hovering = false;
	let pendingDelete = false;
	const dispatch = createEventDispatcher<{ delete: { name: string } }>();

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

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Space' || event.key === 'Enter') pendingDelete = true;
	}

	function handleDelete() {
		pendingDelete = false;
		dispatch('delete', {
			name: save.name
		});
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
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 448 512"
				height="20"
				width="20"
				on:click={() => (pendingDelete = true)}
				on:keydown={handleKeydown}
			>
				<path
					fill="#9a9696"
					d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
				/>
			</svg>
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
	path {
		transition: fill 0.3s;
	}
	path:hover {
		fill: var(--salmon-strong);
		transition: fill 0.15s;
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
