<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { error } from '$lib/stores';

	export let text: string;
	export let active: boolean;

	let input: HTMLInputElement;
	let origText = `${text}`;

	const parseText = (text: string) => text.replace(/[^\w-]+/g, '');

	$: text = parseText(text);

	onMount(() => {
		if (input) input.focus();
	});

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') input.blur();
	}

	// Updates the save name if the new name is valid
	function handleSubmit() {
		active = false;
		let name = parseText(text);
		if (!name) {
			text = origText;
		} else if (origText !== name) {
			invoke<void>('rename_save', {
				old: origText,
				new: name
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
	pattern="[\w-]+$"
	minlength="1"
	maxlength="25"
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
	}
	input:invalid:not(:placeholder-shown) {
		border-color: var(--salmon-strong);
	}
</style>
