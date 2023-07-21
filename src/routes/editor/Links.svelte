<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';

	const tabs = [
		{
			title: 'Layout',
			url: '/setup'
		},
		{
			title: 'Editor',
			url: '/editor'
		},
		{
			title: 'Results',
			url: '/results'
		}
	];
</script>

<div>
	{#each tabs as tab}
		{@const url = `${base}/editor${tab.url}`}
		<a href={url} class="focus-template" class:active={$page.url.pathname === url}>
			{tab.title}
		</a>
	{/each}
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		row-gap: 0.4em;
	}
	a {
		--focus-border-offset: -2px;
		border-radius: 0.5em;
		padding: 0.75em;
		color: var(--gray-mild);
		font-weight: 600;
		transition:
			background-color 0.15s,
			color 0.15s;
	}
	a:is(:hover, :focus-within) {
		color: var(--light);
		background-color: var(--dark-mild);
	}
	a.active {
		color: var(--light);
		background-color: var(--dark-strong);
	}
</style>
