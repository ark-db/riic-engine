<script lang="ts">
	import xmark from '$lib/images/red-xmark.svg';
	export let msg: string;

	let visible = true;
</script>

{#if visible}
	<div class="bottom">
		<div class="container">
			<p>{msg}</p>
			<input
				type="image"
				src={xmark}
				alt="Delete button"
				height="20"
				width="20"
				on:click={() => (visible = false)}
			/>
		</div>
	</div>
{/if}

<style>
	.bottom {
		display: flex;
		justify-content: center;
	}
	.container {
		flex-grow: 1;
		position: fixed;
		bottom: 5%;
		border: 2px solid var(--salmon-strong);
		border-radius: 1em;
		padding: 0 1em;
		background-color: var(--dark-strong);
	}
	p {
		color: var(--salmon-strong);
	}
	input {
		position: absolute;
		top: -15px;
		right: -15px;
		border: 2px solid var(--salmon-strong);
		border-radius: 50%;
		padding: 2.5px;
		background-color: var(--dark-strong);
	}
</style>
