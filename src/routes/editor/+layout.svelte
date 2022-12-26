<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';
	import { activeSave, error } from '$lib/stores';
	import type { ActiveSave } from '$lib/types';
	import logo from '$lib/images/logo.png';
	import menuIcon from '$lib/images/menu.png';
	import GradientContainer from '$lib/components/GradientContainer.svelte';
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
			url: '/editor'
		},
		{
			title: 'Results',
			url: '/results'
		}
	];

	let menuActive = true;
	$: menuIconDesc = `${menuActive ? 'Collapse' : 'Expand'} menu`;

	invoke<void>('rename_window', { name: $activeSave.name });

	let init = true;
	function updateSave(save: ActiveSave) {
		if (!init) invoke<void>('update_save', { save }).catch(error.handle);
	}
	$: updateSave($activeSave);
	onMount(() => (init = false));
</script>

<div class="container">
	{#if menuActive}
		<nav transition:fly|local={{ x: -200, duration: 150 }}>
			<GradientContainer --weight="3px" --radius="0.5em" --bg-color="var(--dark-strong)">
				<a href="/" class="home-link">
					<img src={logo} alt="App logo" width="48" height="48" />
					<p class="app-title">RIIC Engine</p>
				</a>
			</GradientContainer>

			<div class="links">
				{#each tabs as tab}
					{@const url = `${base}/editor${tab.url}`}
					<a href={url} class:active={$page.url.pathname === url}>
						{tab.title}
					</a>
				{/each}
			</div>
		</nav>
	{/if}
	<main>
		<div class="top-bar">
			<Button desc={menuIconDesc} onClick={() => (menuActive = !menuActive)}>
				<img src={menuIcon} alt={menuIconDesc} id="menu-icon" width="32" height="32" />
			</Button>
		</div>
		<div class="content">
			<slot />
		</div>
	</main>
</div>

<style>
	.container {
		min-height: 100vh;
		display: flex;
	}
	nav {
		z-index: 98;
		min-width: 14em;
		max-width: 18em;
		padding: 0.5em;
		background-color: var(--darkish);
		box-shadow: 0 0 0.5em rgba(0, 0, 0, 0.75);
		clip-path: inset(0 -0.5em 0 0);
		display: flex;
		flex-direction: column;
		row-gap: 1em;
		transition: width 0.15s;
	}
	.home-link {
		padding: 0.5em;
		display: flex;
		align-items: center;
		column-gap: 0.75em;
	}
	.home-link:is(:hover, :focus-within) {
		background-color: var(--dark);
	}
	.app-title {
		margin: 0;
		color: var(--light);
		font-weight: 600;
		font-size: 1.5em;
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
	.top-bar {
		z-index: 97;
		position: fixed;
		top: 0;
		width: 100%;
		padding: 0.5em 0;
	}
	#menu-icon {
		border-radius: 5px;
		padding: 2.5px;
		background-color: var(--light);
		box-shadow: 0 0 0.5em rgba(255, 255, 255, 0.75);
		transition: background-color 0.2s;
		filter: invert(1);
	}
	#menu-icon:hover {
		background-color: var(--gray-mild);
	}
	.content {
		margin-top: 3em;
	}
	::-webkit-scrollbar {
		background-color: var(--dark-strong);
	}
	::-webkit-scrollbar-thumb {
		border-radius: 50vh;
		background-color: var(--dark-mild);
	}
</style>
