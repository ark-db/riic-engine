<script>
	import { page } from '$app/stores';
</script>

<div>
	<h1>{$page.status} <span>|</span> {$page.error?.message}</h1>
	<p>An unexpected error occurred.</p>
	<a href="/">Return to main menu</a>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
	h1,
	p,
	a {
		color: var(--light);
		text-align: center;
	}
	h1 {
		border-radius: 0.5em;
		padding: 0.5em;
		background-color: var(--dark-strong);
		font-size: 3em;
	}
	span {
        margin: 0 0.2em;
		font-weight: 100;
	}
	a {
		margin: 2em auto 0;
		border-radius: 0.5em;
		padding: 1em;
		background-color: var(--blue);
		text-decoration: none;
	}
	a:hover {
		background-color: var(--blue-strong);
	}
</style>
