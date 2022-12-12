<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import { activeSave } from '$lib/stores';
	import logo from '$lib/images/logo.png';
	import menuIcon from '$lib/images/menu.png';
	import Button from '$lib/components/Button.svelte';

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

	let menuActive = true;

	$: menuIconDesc = `${menuActive ? 'Collapse' : 'Expand'} menu`;
</script>

<div class="container">
	<nav class:hidden={!menuActive}>
		<a href="/" class="home-link" on:mousedown|preventDefault>
			<img src={logo} alt="App logo" width="48" height="48" />
			<p class="app-title">RIIC Engine</p>
		</a>

		<p class="save-name">{$activeSave.name}</p>

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
		<Button desc={menuIconDesc} onClick={() => (menuActive = !menuActive)}>
			<img src={menuIcon} alt={menuIconDesc} id="menu-icon" width="30" height="30" />
		</Button>
		<slot />
	</main>
</div>

<style>
	.container {
		min-height: 100vh;
		display: flex;
	}
	nav {
		width: clamp(14em, 20%, 18em);
		padding: 0.5em;
		background-color: var(--darkish);
		box-shadow: 0 0 0.5em rgba(0, 0, 0, 0.75);
		clip-path: inset(0 -0.5em 0 0);
		display: flex;
		flex-direction: column;
		row-gap: 0.5em;
		transition: width 0.15s;
	}
	nav.hidden {
		width: 0;
		padding: 0.5em 0;
		box-shadow: none;
		clip-path: none;
	}
	nav,
	nav * {
		overflow: clip;
		white-space: nowrap;
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
		border-left: 0.3em solid var(--blue-mild);
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
		color: var(--gray-mild);
		text-decoration: none;
		font-weight: 600;
		transition: background-color 0.15s, color 0.15s;
	}
	a:hover {
		color: var(--light);
		background-color: var(--dark-mild);
	}
	a.active {
		color: var(--light);
		background-color: var(--dark-strong);
	}
	main {
		flex-grow: 1;
		padding: 0.5em;
		background-color: var(--dark-strong);
		overflow: auto;
	}
	#menu-icon {
		border-radius: 5px;
		padding: 2.5px;
		transition: background-color 0.2s;
		filter: invert(1);
	}
	#menu-icon:hover {
		background-color: var(--gray-mild);
	}
</style>
