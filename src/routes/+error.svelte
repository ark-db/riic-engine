<script lang="ts">
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/tauri';
	import github from '$lib/images/github.svg';
	import lancet from '$lib/images/lancet.avif';
	import { error } from '$lib/stores';

	function openGithub() {
		invoke('open', { url: 'https://github.com/ark-db/riic-engine/issues' }).catch(error.handle);
	}
</script>

<div class="container">
	<h1>{$page.status} <span>|</span> {$page.error?.message ?? 'Unknown error'}</h1>
	<p>An unexpected error occurred.</p>
	<img class="lancet" src={lancet} alt="Lancet-2" width="320" height="320" />
	<div class="links">
		<a href="/">Return to main menu</a>
		<button on:click|trusted={openGithub}>
			Report issue on GitHub <img src={github} alt="GitHub logo" width="32" height="32" />
		</button>
	</div>
</div>

<style>
	.container {
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
	.lancet {
		margin: 0 auto;
	}
	.links {
		margin: auto;
		font-size: 1.1em;
		display: flex;
		column-gap: 5em;
	}
	a,
	button {
		width: 17em;
		height: 3.75em;
		border-radius: 0.5em;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	a {
		background-color: var(--blue);
		text-decoration: none;
		&:hover {
			background-color: var(--blue-strong);
		}
	}
	button {
		background-color: var(--light);
		column-gap: 1.5em;
		&:hover {
			background-color: var(--light-strong);
		}
	}
</style>
