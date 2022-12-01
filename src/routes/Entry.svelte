<script lang="ts">
	import type { FileData } from '$lib/types';
	import { saveList, saveSortMode, activeSave } from '$lib/stores';
	import { interaction } from '$lib/utils';
	import { tooltip } from '$lib/tooltip';
	import NameInput from './NameInput.svelte';
	import Modal from '$lib/components/Modal.svelte';
	export let save: FileData;

	let hovering = false;
	let pendingRename = false;
	let pendingDelete = false;

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

	const handleSelect = interaction(() => activeSave.load(save.name));

	const handleRename = interaction(() => (pendingRename = true));

	const handleExport = interaction(() => saveList.export(save.name));

	const handleDeleteAction = interaction(() => (pendingDelete = true));

	function handleDelete() {
		pendingDelete = false;
		saveList.delete(save.name);
	}
</script>

<div
	class="container"
	on:mouseenter={() => (hovering = true)}
	on:mouseleave={() => (hovering = false)}
>
	{#if pendingRename}
		<NameInput bind:text={save.name} bind:active={pendingRename} />
	{:else}
		<button class="entry-title" on:click={handleSelect} on:keydown={handleSelect}>
			{save.name}
		</button>
	{/if}
	<div class="right">
		{#if hovering}
			<div class="settings">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 512 512"
					height="24"
					width="24"
					use:tooltip={'Rename save'}
					on:click={handleRename}
					on:keydown={handleRename}
				>
					<path
						class="edit-title"
						d="M43 16C28.0656 16 16 28.0656 16 43C16 57.9344 28.0656 70 43 70L151 70L151 367C151 381.934 163.066 394 178 394C192.934 394 205 381.934 205 367L205 70L313 70C327.934 70 340 57.9344 340 43C340 28.0656 327.934 16 313 16L178 16L43 16Z"
						fill="#9a9696"
					/>
					<path
						class="edit-title"
						d="M421.544 249.384L397.318 273.609L462.388 338.675L486.615 314.45C499.128 301.938 499.128 281.667 486.615 269.154L466.893 249.384C454.38 236.872 434.108 236.872 421.594 249.384L421.544 249.384ZM386.005 284.92L269.328 401.638C264.123 406.843 260.319 413.3 258.216 420.357L240.497 480.568C239.246 484.822 240.397 489.377 243.5 492.48C246.604 495.583 251.159 496.734 255.363 495.533L315.579 477.815C322.636 475.713 329.093 471.909 334.299 466.704L451.076 349.986C451.076 349.986 386.005 284.92 386.005 284.92Z"
						fill="#9a9696"
					/>
				</svg>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 640 512"
					height="26"
					width="26"
					use:tooltip={'Export save'}
					on:click={handleExport}
					on:keydown={handleExport}
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
					use:tooltip={'Delete save'}
					on:click={handleDeleteAction}
					on:keydown={handleDeleteAction}
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
		height: 2.5em;
		border-radius: 0.75em;
		padding: 1em;
		background: var(--dark-strong);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.entry-title {
		margin-left: 0.1em;
		border: none;
		padding: 0.5em;
		background-color: var(--dark-strong);
		color: var(--light);
		font-size: 1.25em;
		font-weight: 600;
	}
	.entry-title:hover {
		cursor: pointer;
	}
	.right {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.settings {
		margin-right: 5px;
		display: flex;
		align-items: center;
		column-gap: 15px;
	}
	path {
		transition: fill 0.3s;
	}
	svg:hover path {
		transition: fill 0.15s;
	}
	svg:hover .export,
	svg:hover .edit-title {
		fill: var(--light);
	}
	svg:hover .trash {
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
