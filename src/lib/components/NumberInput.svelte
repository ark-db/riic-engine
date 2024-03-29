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

	function handleKeydown({ key }: KeyboardEvent) {
		if (key === 'Enter') input.blur();
	}

	function isValid(value: unknown): boolean {
		return typeof value === 'number' && value >= min && value <= max;
	}

	// Runs the `onValidInput` callback if the new value is valid and different from the old value
	export function handleSubmit() {
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
		bind:value={qty}
		bind:this={input}
		on:keydown|trusted={handleKeydown}
		on:blur={handleSubmit}
	/>
</div>

<style>
	div {
		display: flex;
		align-items: center;
		/* --font-size is defined in the parent component */
		column-gap: calc(var(--font-size) / 1.8);
	}
	input {
		font-size: var(--font-size);
		&.invalid:not(:placeholder-shown) {
			border-color: var(--salmon-strong);
		}
	}
	input::placeholder {
		font-size: 0.9em;
	}
</style>
