<script>
	import logo from '$lib/images/logo.png';
</script>

<header>
	<img src={logo} alt="App logo" height="128" width="128" />
	<div>
		<h1>RIIC Engine</h1>
		<p>astralsky • v0.1.0</p>
	</div>
</header>

<style>
	header {
		padding: 1em;
		background-color: var(--dark-strong);
		display: flex;
		align-items: center;
		gap: 1em;
	}
	h1 {
		color: var(--light);
		font-size: 4em;
		margin: 0;
	}
	p {
		color: var(--gray);
	}
</style>
