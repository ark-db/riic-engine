<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import GradientContainer from '$lib/components/GradientContainer.svelte';
	import Button from '$lib/components/Button.svelte';
	import logo from '$lib/images/logo.webp';
	import menuIcon from '$lib/images/menu.webp';
	import { activeSave, error } from '$lib/stores';
	import type { ActiveSave } from '$lib/types';
	import Links from './Links.svelte';
	import ShiftCount from './ShiftCount.svelte';
	import ShiftInterval from './ShiftInterval.svelte';
	import Drones from './Drones.svelte';
	import PowerUsage from './PowerUsage.svelte';

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
			<GradientContainer weight={3} radius={8} bgColor="var(--dark-strong)">
				<a href="/" class="home-link">
					<img src={logo} alt="App logo" width="48" height="48" />
					<p class="app-title">RIIC Engine</p>
				</a>
			</GradientContainer>
			<Links />
		</nav>
	{/if}
	<section class="top-bar" class:nav-hidden={!menuActive}>
		<Button desc={menuIconDesc} onClick={() => (menuActive = !menuActive)}>
			<img src={menuIcon} alt={menuIconDesc} id="menu-icon" width="32" height="32" />
		</Button>
		<div class="stats">
			<ShiftCount />
			<ShiftInterval />
			<Drones />
			<PowerUsage />
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
		grid-template-rows: minmax(3.5em, 6.25vh) 1fr;
		grid-template-columns: 15em 1fr;
	}
	nav {
		grid-row: span 2;
		grid-column: 1 / 2;
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
		border-radius: 0.5em;
		padding: 0.5em;
		display: flex;
		align-items: center;
		column-gap: 0.75em;
		transition: background-color 0.15s, color 0.15s;
	}
	.home-link:is(:hover, :focus-within) {
		outline: none;
		background-color: var(--dark);
	}
	.app-title {
		margin: 0;
		color: var(--light);
		font-weight: 600;
		font-size: 1.5em;
	}
	.top-bar {
		--font-size: clamp(1.25em, 2.5vh, 1.5em);
		grid-row: 1 / 2;
		grid-column: 2 / 3;
		padding: 0.5em;
		align-self: center;
		display: flex;
		align-items: center;
		justify-content: space-between;
		column-gap: 1em;
	}
	.nav-hidden {
		grid-column: span 2;
	}
	#menu-icon {
		--icon-size: clamp(3em, 3.75vh, 3.75vh);
		width: var(--icon-size);
		height: var(--icon-size);
		margin: 0.25em 0.25em 0;
		border-radius: 0.5em;
		padding: calc(var(--icon-size) / 8);
		background-color: var(--dark-strong);
		transition: all 0.2s;
	}
	#menu-icon:hover {
		background-color: var(--dark-mild);
		box-shadow: 0 0 0.25em var(--dark-mild);
	}
	.stats {
		display: flex;
		align-items: center;
		column-gap: 0.5em;
	}
	main {
		grid-row: 2 / 3;
		grid-column: 2 / 3;
		position: relative;
		padding: 0.75em;
		background: url('$lib/images/contours.webp') no-repeat;
		background-size: cover;
		overflow: auto;
	}
</style>
