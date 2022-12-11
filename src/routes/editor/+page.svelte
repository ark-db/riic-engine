<script>
	import { activeSave } from '$lib/stores';
</script>

<p>Control Center level: {$activeSave.data.layout.cc.level}</p>

<style>
	p {
		color: var(--light);
	}
</style>
