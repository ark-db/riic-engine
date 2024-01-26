<script>
	import logo from '$lib/images/logo.avif';
</script>

<header>
	<img src={logo} alt="App logo" width="128" height="128" />
	<div>
		<h1>RIIC Engine</h1>
		<p>astralsky • v0.1.0</p>
	</div>
</header>

<style>
	header {
		padding: 1em;
		background: var(--dark-strong);
		display: flex;
		align-items: center;
		gap: 1em;
	}
	h1 {
		color: var(--light);
		font-size: 4em;
	}
	p {
		color: var(--gray);
	}
</style>
