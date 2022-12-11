<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import { activeSaveTitle } from '$lib/stores';
	import logo from '$lib/images/logo.png';

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
		<a href="/" class="home-link" on:mousedown|preventDefault>
			<img src={logo} alt="App logo" width="48" height="48" />
			<p class="app-title">RIIC Engine</p>
		</a>

		<p class="save-name">{$activeSaveTitle}</p>

		<div class="links">
			{#each tabs as tab}
				{@const url = `${base}/editor${tab.url}`}
				<a href={url} class:active={$page.url.pathname === url} on:mousedown|preventDefault>
					{tab.title}
				</a>
			{/each}
		</div>
	</nav>
	<main>
		<slot />
	</main>
</div>

<style>
	.container {
		min-height: calc(100vh - 1em);
		display: flex;
		column-gap: 0.5em;
	}
	nav {
		width: clamp(14em, 20%, 18em);
		background-color: var(--dark);
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
	}
	.home-link {
		padding: 0.5em;
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
		column-gap: 0.75em;
	}
	.app-title {
		margin: 0;
		color: var(--light);
		font-weight: 600;
		font-size: 1.5em;
	}
	.save-name {
		margin: 0;
		border-left: 0.3em solid var(--light);
		border-radius: 0 0.5em 0.5em 0;
		padding: 0.75em;
		background-color: var(--dark-strong);
		color: var(--light-strong);
		text-align: center;
		font-weight: 600;
	}
	.links {
		display: flex;
		flex-direction: column;
		row-gap: 0.3em;
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
	main {
		flex-grow: 1;
		border-radius: 0.5em;
		background-color: var(--dark-strong);
		overflow: auto;
	}
</style>
