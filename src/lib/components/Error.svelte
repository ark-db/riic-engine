<script lang="ts">
	import xmark from '$lib/images/ui/red-xmark.svg';
	import { error } from '$lib/stores';

	let dialog: HTMLDialogElement;

	$: if (dialog) $error ? dialog.show() : dialog.close();
</script>

<dialog
	role="alertdialog"
	aria-label="Error message"
	aria-describedby="error-message"
	bind:this={dialog}
>
	<p id="error-message">{$error}</p>
	<button class="focus-template" aria-label="Dismiss error" on:click|trusted={error.clear}>
		<img src={xmark} alt="Red X mark" width="20" height="20" />
	</button>
</dialog>

<style>
	dialog {
		--border-weight: 2px;
		z-index: 2;
		position: fixed;
		bottom: 7.5%;
		margin: auto;
		box-shadow: 0 6px 6px rgb(0 0 0 / 0.5);
		border: var(--border-weight) solid var(--salmon-strong);
		border-radius: 0.75em;
		padding: 1.25em;
		background-color: var(--dark-strong);
	}
	p {
		color: var(--salmon-strong);
		font-size: 1.05em;
	}
	button {
		--focus-border-offset: -6px;
		--focus-border-radius: 50%;
		--button-icon-size: 20px;
		--button-padding: 2.5px;
		--button-size: calc(
			var(--button-icon-size) + var(--button-padding) * 2 + var(--border-weight) * 4
		);
		--button-shift: calc(0px - var(--button-size) / 2);
		position: absolute;
		top: var(--button-shift);
		right: var(--button-shift);
		border: var(--border-weight) solid var(--salmon-strong);
		border-radius: 50%;
		padding: var(--button-padding);
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
	}
</style>
