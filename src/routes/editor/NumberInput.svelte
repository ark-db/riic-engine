<script lang="ts">
	import { error } from '$lib/stores';
	import { tooltip } from '$lib/tooltip';

	export let desc: string;
	export let min: number;
	export let max: number;
	export let initial: number;
	export let placeholder: string;
	export let onValidInput: (value: number) => void;
	export let errorMsg: string;
	export let iconSrc: string;
	export let iconSize: number;

	let input: HTMLInputElement;
	let qty = initial;
	$: invalid = !isValid(qty);

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') input.blur();
	}

	function isValid(value: unknown): boolean {
		return typeof value === 'number' && value >= min && value <= max;
	}

	function handleSubmit() {
		if (invalid) {
			if (qty !== null) {
				error.handle(errorMsg);
			}
			qty = initial;
		} else {
			if (initial !== qty) onValidInput(qty);
			error.clear();
		}
	}
</script>

<div>
	<img use:tooltip={desc} src={iconSrc} alt={desc} width={iconSize} height={iconSize} />
	<input
		class="input-template"
		class:invalid
		aria-label={desc}
		type="number"
		{placeholder}
		required
		pattern="[0-9]+"
		{min}
		{max}
		step="1"
		autocapitalize="off"
		autocomplete="off"
		spellcheck="false"
		enterkeyhint="done"
		bind:this={input}
		bind:value={qty}
		on:keydown|trusted={handleKeydown}
		on:blur={handleSubmit}
	/>
</div>

<style>
	div {
		padding: 0 0.5em;
		display: flex;
		align-items: center;
		column-gap: 0.5em;
	}
	input {
		font-size: var(--font-size);
	}
	input.invalid:not(:placeholder-shown) {
		border-color: var(--salmon-strong);
	}
	::placeholder {
		font-size: 0.9em;
	}
</style>
