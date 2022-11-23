<script>
	import { activeSave } from '@stores';
</script>

<p>Title: {$activeSave.title}</p>
<p>Version: {$activeSave.data.version.major}.{$activeSave.data.version.minor}</p>

<style>
	p {
		color: var(--light);
	}
</style>
