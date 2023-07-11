<script lang="ts">
	import { page } from '$app/stores';
	import src from '$lib/images/lancet.webp';
</script>

<div>
	<h1>{$page.status} <span>|</span> {$page.error?.message ?? 'Unknown error'}</h1>
	<p>An unexpected error occurred.</p>
	<img {src} alt="Lancet-2" width="320" height="320" />
	<a href="/">Return to main menu</a>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		justify-content: center;
		row-gap: 2em;
	}
	h1,
	p,
	a {
		color: var(--light);
		text-align: center;
	}
	h1 {
		padding: 0.75em;
		background-color: var(--dark-strong);
		font-size: 3em;
	}
	span {
		margin: 0 0.2em;
		font-weight: 100;
	}
	img,
	a {
		margin: 0 auto;
	}
	a {
		border-radius: 0.5em;
		padding: 1em;
		background-color: var(--blue);
		text-decoration: none;
	}
	a:hover {
		background-color: var(--blue-strong);
	}
</style>
