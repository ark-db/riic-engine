<script lang="ts">
	import GradientContainer from '$lib/components/GradientContainer.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import { saveList, saveSortMode, activeSave } from '$lib/stores';
	import type { SaveTimeData } from '$lib/types';
	import NameInput from './NameInput.svelte';
	import EntrySettings from './EntrySettings.svelte';

	export let save: SaveTimeData;

	let container: HTMLLIElement;

	let hovering = false;
	let focused = false;
	let pendingRename = false;
	let pendingDelete = false;

	// Checks if this component instance contains the currently-focused element. Works with keyboard navigation!
	function handleFocusChange({ type, target, relatedTarget }: FocusEvent) {
		type FocusEventType = 'focusin' | 'focusout';

		// Test different target element depending on the event type
		const focusTarget = (type as FocusEventType) === 'focusin' ? target : relatedTarget;
		focused = focusTarget instanceof Node && container.contains(focusTarget);
	}

	// Formats an elapsed duration (in seconds) as a human-readable string
	function formatElapsedTime(secs: number): string {
		if (secs === 0) {
			return 'Just now';
		} else if (secs < 60) {
			return formatStr(secs, 'second');
		} else if (secs < 60 * 60) {
			return formatStr(Math.floor(secs / 60), 'minute');
		} else if (secs < 60 * 60 * 24) {
			return formatStr(Math.floor(secs / (60 * 60)), 'hour');
		} else {
			return formatStr(Math.floor(secs / (60 * 60 * 24)), 'day');
		}
	}

	function formatStr(num: number, unit: string): string {
		return `${num} ${unit}${num === 1 ? '' : 's'} ago`;
	}

	// Deletes a save file
	function handleDelete() {
		pendingDelete = false;
		saveList.delete(save.name);
		hovering = false;
	}

	function handleDeleteKeydown({ key }: KeyboardEvent) {
		if (pendingDelete && key === 'Enter') handleDelete();
	}
</script>

<svelte:window on:keydown|trusted={handleDeleteKeydown} />

<GradientContainer weight={3} radius={12} bgColor="var(--dark-strong)">
	<li
		class="container"
		bind:this={container}
		on:mouseenter|trusted={() => (hovering = true)}
		on:mouseleave|trusted={() => (hovering = false)}
		on:focusin|trusted={handleFocusChange}
		on:focusout|trusted={handleFocusChange}
	>
		{#if pendingRename}
			<NameInput bind:text={save.name} bind:active={pendingRename} />
		{:else}
			<button
				class="focus-template entry-title"
				on:click|trusted={() => activeSave.load(save.name)}
			>
				{save.name}
			</button>
		{/if}
		<div class="right">
			{#if hovering || focused}
				<EntrySettings
					onRename={() => (pendingRename = true)}
					onExport={() => saveList.export(save.name)}
					onDelete={() => (pendingDelete = true)}
				/>
			{:else}
				<p class="time">
					{formatElapsedTime(Math.round(save[$saveSortMode.mode]))}
				</p>
			{/if}
		</div>
	</li>
</GradientContainer>

{#if pendingDelete}
	<Modal let:modal label="Confirm save deletion" onClose={() => (pendingDelete = false)}>
		<p class="modal-text">Are you sure you want to delete this save file?</p>
		<div class="modal-options">
			<button
				class="focus-template cancel"
				on:click|trusted={() => {
					modal.close();
					pendingDelete = false;
				}}
			>
				Cancel
			</button>
			<button
				class="focus-template delete"
				on:click|trusted={() => {
					modal.close();
					handleDelete();
				}}
			>
				Delete
			</button>
		</div>
	</Modal>
{/if}

<style>
	.container {
		height: 2.5em;
		padding: 1em;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.entry-title {
		--focus-border-offset: -4px;
		margin-left: 0.1em;
		padding: 0.5em;
		background-color: var(--dark-strong);
		color: var(--light);
		font-size: 1.25em;
		font-weight: 600;
	}
	.right {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.time {
		padding: 0 0.5em;
		color: var(--gray);
	}
	.modal-text {
		color: var(--light);
	}
	.modal-options {
		margin-top: 1.5em;
		display: flex;
		column-gap: 1em;
	}
	.modal-options > button {
		--focus-border-offset: -4px;
		border: none;
		border-radius: 0.5em;
		padding: 0.75em;
		font-size: 0.8em;
		font-weight: 600;
	}
	.cancel {
		background-color: var(--light);
	}
	.delete {
		background-color: var(--salmon-strong);
		color: var(--light);
	}
</style>
