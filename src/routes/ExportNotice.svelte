<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import check from '$lib/images/ui/check.svg';
	import xmark from '$lib/images/ui/white-xmark.svg';
	import { exportNotice } from '$lib/stores';

	beforeNavigate(exportNotice.deactivate);
</script>

{#if $exportNotice}
	<div class="export-notice" role="alert" transition:fly={{ duration: 300, x: 200 }}>
		<img src={check} alt="Check mark" width="32" height="32" />
		<p>Successfully exported save to the Downloads folder!</p>
		<button
			class="focus-template"
			aria-label="Dismiss alert"
			on:click|trusted={exportNotice.deactivate}
		>
			<img src={xmark} alt="White X mark" width="16" height="16" />
		</button>
	</div>
{/if}

<style>
	.export-notice {
		position: fixed;
		top: 0;
		right: 0;
		margin: 0.5em;
		box-shadow: -3px 6px 6px rgba(0 0 0 / 0.5);
		border: 2px solid #43a047;
		border-radius: 0.5em;
		background-color: var(--dark-strong);
		padding: 1em;
		display: flex;
		align-items: center;
		column-gap: 1em;
	}
	p {
		margin-right: 1em;
		color: var(--light-strong);
		font-size: 1.05em;
	}
	button {
		--focus-border-offset: -5px;
		padding: 4px;
	}
</style>
