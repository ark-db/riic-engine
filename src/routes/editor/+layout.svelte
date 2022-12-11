<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import { activeSaveTitle } from '$lib/stores';

	type NavLink = {
		title: string;
		url: string;
	};

	const tabs: NavLink[] = [
		{
			title: 'Layout',
			url: '/setup'
		},
		{
			title: 'Editor',
			url: ''
		},
		{
			title: 'Results',
			url: '/results'
		}
	];

	invoke<void>('rename_window', {
		name: $activeSaveTitle
	});
</script>

<div class="container">
	<nav>
		<p>{$activeSaveTitle}</p>
		{#each tabs as tab}
			{@const url = `${base}/editor${tab.url}`}
			<a href={url} class:active={$page.url.pathname === url} on:mousedown|preventDefault>
				{tab.title}
			</a>
		{/each}
		<a href="/">Return to main menu</a>
	</nav>
	<main>
		<slot />
	</main>
</div>

<style>
	.container {
		display: flex;
		column-gap: 0.5em;
	}
	nav {
		width: clamp(12em, 20%, 18em);
		background-color: var(--dark);
		display: flex;
		flex-direction: column;
		row-gap: 0.25em;
	}
	p {
		color: var(--gray);
	}
	a {
		border-radius: 0.5em;
		padding: 0.75em;
		color: var(--light);
		text-decoration: none;
		font-weight: 600;
		transition: background-color 0.15s;
	}
	a:hover {
		background-color: var(--dark-mild);
	}
	a.active {
		background-color: var(--dark-strong);
	}
</style>
