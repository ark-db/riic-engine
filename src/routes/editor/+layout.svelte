<script lang="ts">
	import GradientContainer from '$lib/components/GradientContainer.svelte';
	import Button from '$lib/components/Button.svelte';
	import logo from '$lib/images/logo.webp';
	import menuIcon from '$lib/images/ui/menu.webp';
	import Links from './Links.svelte';
	import ShiftCount from './ShiftCount.svelte';
	import ShiftInterval from './ShiftInterval.svelte';
	import Drones from './Drones.svelte';
	import PowerUsage from './PowerUsage.svelte';

	let menuActive = true;
	$: menuIconDesc = `${menuActive ? 'Hide' : 'Show'} editor menu`;
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
			<img src={menuIcon} alt={menuIconDesc} class="menu-icon" width="32" height="32" />
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
		transition:
			background-color 0.15s,
			color 0.15s;
	}
	.home-link:is(:hover, :focus-within) {
		outline: none;
		background-color: var(--dark);
	}
	.app-title {
		color: var(--light);
		font-weight: 600;
		font-size: 1.5em;
	}
	.top-bar {
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
	.menu-icon {
		margin-left: 2px;
		border-radius: 0.5em;
		padding: 6px;
		background-color: var(--dark-strong);
		transition: all 0.2s;
	}
	.menu-icon:hover {
		background-color: var(--dark-mild);
		box-shadow: 0 0 0.25em var(--dark-mild);
	}
	.stats {
		/* --font-size is used in ./NumberInput.svelte */
		--font-size: clamp(1.25em, 2vh, 1.4em);
		display: flex;
		align-items: center;
		column-gap: 1.5em;
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
