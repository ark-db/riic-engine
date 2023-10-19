<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { error } from '$lib/stores';

	export let text: string;
	export let active: boolean;

	let input: HTMLInputElement;

	let origText = `${text}`;

	onMount(() => {
		if (input) input.focus();
	});

	function handleKeydown({ key }: KeyboardEvent) {
		if (key === 'Enter') input.blur();
	}

	// Updates the save name if the new name is valid and different from the old name
	function handleSubmit() {
		active = false;
		if (!text) {
			text = origText;
		} else if (origText !== text) {
			invoke<void>('rename_save', {
				old: origText,
				new: text
			})
				.then(() => {
					origText = text;
					error.clear();
				})
				.catch((err) => {
					text = origText;
					error.handle(err);
				});
		}
	}
</script>

<input
	class="input-template"
	aria-label="Save name"
	type="text"
	placeholder="Setup name..."
	required
	minlength="1"
	maxlength="30"
	autocapitalize="off"
	autocomplete="off"
	spellcheck="false"
	enterkeyhint="done"
	bind:this={input}
	bind:value={text}
	on:keydown|trusted={handleKeydown}
	on:focusout={handleSubmit}
/>

<style>
	input {
		padding-bottom: 0.3em;
		font-size: 1.25em;
		&:invalid:not(:placeholder-shown) {
			border-color: var(--salmon-strong);
		}
	}
</style>
