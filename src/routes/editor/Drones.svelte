<script lang="ts">
	import drones from '$lib/images/drones.webp';
	import { activeSave, error } from '$lib/stores';

	const min = 0;
	const max = 999999;

	let input: HTMLInputElement;
	let qty = $activeSave.data.drones;
	$: invalid = !isValid(qty);

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') input.blur();
	}

	function isValid(value: unknown): boolean {
		return typeof value === 'number' && min <= value && value <= max;
	}

	function handleSubmit() {
		if (invalid) {
			qty = $activeSave.data.drones;
			error.handle('The specified drone quantity should be a number from 0 to 999999');
		} else {
			if ($activeSave.data.drones !== qty) {
				$activeSave.data.drones = qty;
			}
			error.clear();
		}
	}
</script>

<div class="container">
	<img src={drones} alt="Drone icon" width="33" height="33" />
	<input
		class:invalid
		type="number"
		placeholder="Drones..."
		required={true}
		pattern="[0-9]+"
		{min}
		{max}
		step="1"
		autocomplete="off"
		enterkeyhint="done"
		bind:this={input}
		bind:value={qty}
		on:keydown|trusted={handleKeydown}
		on:blur={handleSubmit}
	/>
</div>

<style>
	.container {
		padding: 0 0.5em;
		display: flex;
		align-items: center;
		column-gap: 0.25em;
	}
	input {
		margin: 0;
		padding: 0.3em 0.5em;
		font-size: clamp(1.25em, 2.5vh, 1.5em);
		font-weight: 600;
		color: var(--light);
		background-color: var(--dark-strong);
		appearance: textfield;
		-moz-appearance: textfield;
	}
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
	input:focus {
		outline: none;
		border: 2px solid var(--light);
		border-radius: 0.25em;
		background-color: var(--dark);
	}
	.invalid:not(:placeholder-shown) {
		border: 2px solid var(--salmon-strong);
		border-radius: 0.25em;
	}
	::placeholder {
		font-size: 0.9em;
		color: var(--gray);
	}
</style>
