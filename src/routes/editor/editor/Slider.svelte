<script lang="ts">
	export let value: number;
	export let min: number;
	export let max: number;
	export let step: number;
</script>

<input type="range" bind:value {min} {max} {step} />

<style>
	input {
		--track-height: 0.4em;
		--track-border-radius: calc(var(--track-height) / 2);
		--track-color: var(--gray);
		--thumb-width: 1em;
		--thumb-height: 2em;
		--thumb-border-radius: 0.25em;
		--thumb-color: var(--light-strong);
		--focus-outline: 2.5px solid var(--blue-mild);
		--focus-offset: 2px;
		appearance: none;
		background: transparent;
		cursor: pointer;
		width: 15em;
	}
	/* Track styles */
	input::-webkit-slider-runnable-track {
		height: var(--track-height);
		border-radius: var(--track-border-radius);
		background: var(--track-color);
	}
	input::-moz-range-track {
		height: var(--track-height);
		border-radius: var(--track-border-radius);
		background: var(--track-color);
	}
	/* Thumb styles */
	input::-webkit-slider-thumb {
		appearance: none;
		height: var(--thumb-height);
		width: var(--thumb-width);
		margin-top: calc(var(--track-height) / 2 - var(--thumb-height) / 2);
		border-radius: var(--thumb-border-radius);
		background-color: var(--thumb-color);
	}
	input::-moz-range-thumb {
		height: var(--thumb-height);
		width: var(--thumb-width);
		border: none;
		border-radius: var(--thumb-border-radius);
		background-color: var(--thumb-color);
	}
	/* Focus styles */
	input:focus {
		outline: none;
	}
	input:focus::-webkit-slider-thumb {
		outline: var(--focus-outline);
		outline-offset: var(--focus-offset);
	}
	input:focus::-moz-range-thumb {
		outline: var(--focus-outline);
		outline-offset: var(--focus-offset);
	}
</style>
