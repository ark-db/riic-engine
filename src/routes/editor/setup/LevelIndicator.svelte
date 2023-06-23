<script lang="ts">
	export let level: number;
</script>

<div class="container">
	<!-- eslint-disable-next-line @typescript-eslint/no-unused-vars -->
	{#each { length: level } as _}
		<div>
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
		/* --facility-color is defined in ./Facility.svelte */
		border-bottom: 7.5px solid var(--facility-color);
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
	}
	.middle {
		width: 10px;
		height: 20px;
		background-color: var(--facility-color);
	}
	.bottom {
		width: 0;
		border-top: 7.5px solid var(--facility-color);
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
	}
</style>
