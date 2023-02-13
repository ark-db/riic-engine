<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import GradientContainer from '$lib/components/GradientContainer.svelte';
	import Button from '$lib/components/Button.svelte';
	import logo from '$lib/images/logo.png';
	import menuIcon from '$lib/images/menu.png';
	import powerIcon from '$lib/images/power.png';
	import slash from '$lib/images/slash.png';
	import { activeSave, error, powerUsage, maxPower } from '$lib/stores';
	import type { ActiveSave } from '$lib/types';

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

	// Rename the app window when moving from the main menu to the editor
	invoke<void>('rename_window', { name: $activeSave.name });

	// The following code doesn't work:
	// $: invoke<void>('update_save', { save: $activeSave }).catch(error.handle);
	// because it will also run when the editor is initially loading the save and no changes have been made yet.
	// The last-modified time will be updated, which is not wanted.
	// The code below will not update the save during initial load which resolves the issue.
	let init = true;
	function updateSave(save: ActiveSave) {
		if (!init) invoke<void>('update_save', { save }).catch(error.handle);
	}
	$: updateSave($activeSave);
	onMount(() => (init = false));
</script>

<div class="container">
	{#if menuActive}
		<nav>
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
	<section class="top-bar" class:nav-hidden={!menuActive}>
		<Button desc={menuIconDesc} onClick={() => (menuActive = !menuActive)}>
			<img src={menuIcon} alt={menuIconDesc} id="menu-icon" width="32" height="32" />
		</Button>
		<div class="power-usage">
			<img src={powerIcon} alt="Power usage of base facilities" width="34" height="34" />
			<div class="text">
				<p class="power-stats">
					<span class:invalid={$powerUsage < 0 || $powerUsage > $maxPower}>
						{$powerUsage}
					</span>
				</p>
				<img class="power-divider" src={slash} alt="slash" width="13" height="31" />
				<p class="power-stats">{$maxPower}</p>
			</div>
		</div>
	</section>
	<main class:nav-hidden={!menuActive}>
		<slot />
	</main>
</div>

<style>
	.container {
		height: 100vh;
		display: grid;
		grid-template-rows: minmax(3.5em, 6vh) 1fr;
		grid-template-columns: 15em 1fr;
	}
	nav {
		grid-row: span 2;
		grid-column: 1 / 2;
		z-index: 98;
		padding: 0.5em;
		background-color: var(--darkish);
		box-shadow: 0 0 0.5em rgb(0 0 0 / 0.75);
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
	.top-bar {
		grid-row: 1 / 2;
		grid-column: 2 / 3;
		padding: 0.5em;
		align-self: center;
		display: flex;
		justify-content: space-between;
	}
	.top-bar.nav-hidden {
		grid-column: span 2;
	}
	#menu-icon {
		margin: 0.25em 0.25em 0;
		border-radius: 5px;
		padding: 2.5px;
		background-color: var(--light);
		box-shadow: 0 0 0.5em rgb(255 255 255 / 0.75);
		transition: background-color 0.2s;
		filter: invert(1);
	}
	#menu-icon:hover {
		background-color: var(--gray-mild);
	}
	.power-usage {
		padding: 0 0.5em;
		color: var(--light);
		display: flex;
		align-items: center;
		justify-content: space-between;
		column-gap: 0.5em;
	}
	.power-usage .text {
		display: flex;
		align-items: center;
	}
	.power-stats {
		margin: 0;
		font-size: clamp(1.25em, 2.5vh, 1.5em);
		font-weight: 600;
	}
	.power-stats .invalid {
		color: var(--salmon-strong);
	}
	.power-divider {
		margin: 0 0.2em;
		--height: clamp(30px, 3.5vh, 40px);
		height: var(--height);
		width: calc(var(--height) / 2.5);
	}
	main {
		grid-row: 2 / 3;
		grid-column: 2 / 3;
		background-color: var(--dark-strong);
		overflow: auto;
		padding: 0.75em;
	}
	main.nav-hidden {
		grid-column: span 2;
	}
</style>
