<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Error from '../Error.svelte';
	export let text: string;

	let origText = `${text}`;
	let input: HTMLInputElement;
	let invalid = false;
	let errMessage = '';

	$: text = parseText(text);

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === 'Escape') input.blur();
	}

	const parseText = (text: string) => text.replace(/[^\w-]+$/, '');

	async function updateText() {
		invalid = false;
		errMessage = '';
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
				})
				.catch((reason) => {
					invalid = true;
					errMessage = String(reason);
					text = origText;
				});
		}
	}
</script>

<input
	type="text"
	placeholder="Setup name..."
	required={true}
	pattern="[\w-]+$"
	minlength="1"
	maxlength="25"
	autocapitalize="off"
	autocomplete="off"
	spellcheck="false"
	enterkeyhint="done"
	bind:this={input}
	bind:value={text}
	on:keydown={handleKeydown}
	on:blur={updateText}
/>

<Error msg={errMessage} visible={invalid} />

<style>
	input {
		margin: 0;
		padding: 0.3em 0.5em 0.4em;
		font-size: 1.25em;
		font-weight: 600;
		color: var(--light);
		background-color: var(--dark-strong);
	}
	input:focus {
		outline: none;
		border: 2px solid var(--light);
		border-radius: 0.25em;
		background-color: var(--dark);
	}
	input:invalid:not(:placeholder-shown) {
		border: 2px solid var(--salmon-strong);
		border-radius: 0.25em;
	}
	::placeholder {
		color: var(--gray);
	}
</style>
