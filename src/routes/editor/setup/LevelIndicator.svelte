<script lang="ts">
	export let level: number;
</script>

<div class="container">
	{#each { length: level } as _}
		<div class="hexagon">
			<div class="top" />
			<div class="middle" />
			<div class="bottom" />
		</div>
	{/each}
</div>

<style>
	.container {
		display: flex;
		column-gap: 2.5px;
	}
	.top {
		width: 0;
		border-bottom: 7.5px solid var(--color);
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
	}
	.middle {
		width: 10px;
		height: 20px;
		background-color: var(--color);
	}
	.bottom {
		width: 0;
		border-top: 7.5px solid var(--color);
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
	}
</style>
