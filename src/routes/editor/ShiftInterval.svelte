<script lang="ts">
	import NumberInput from '$lib/components/NumberInput.svelte';
	import clock from '$lib/images/ui/clock.svg';
	import { activeSave } from '$lib/stores';
</script>

<NumberInput
	desc="Shift duration in minutes"
	min={0}
	max={9999}
	initial={$activeSave.interval}
	placeholder="Time"
	onValidInput={(value) => ($activeSave.interval = value)}
	errorMsg="The specified shift duration should be a number from 0 to 9999"
	iconSrc={clock}
	iconSize={30}
/>
