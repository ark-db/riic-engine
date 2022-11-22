<script>
	import { activeSave } from '@stores';
	import Error from '$lib/components/Error.svelte';
</script>

<p>Title: {$activeSave.title}</p>
<p>Version: {$activeSave.data.version.major}.{$activeSave.data.version.minor}</p>

<style>
	p {
		color: var(--light);
	}
</style>
