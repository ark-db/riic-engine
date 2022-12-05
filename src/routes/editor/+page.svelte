<script>
	import { activeSaveTitle, activeSave } from '$lib/stores';
</script>

<p>Title: {$activeSaveTitle}</p>
<p>Control Center level: {$activeSave.layout.cc.level}</p>
<a href="/">Go back</a>

<style>
	p {
		color: var(--light);
	}
</style>
