<script lang="ts">
	import clock from '$lib/images/clock.svg';
	import { activeSave } from '$lib/stores';
	import NumberInput from './NumberInput.svelte';
</script>

<NumberInput
	min={0}
	max={9999}
	initial={$activeSave.data.interval}
	placeholder="Time"
	onValidInput={(value) => ($activeSave.data.interval = value)}
	errorMsg="The specified shift duration should be a number from 0 to 9999"
	iconSrc={clock}
	iconDesc="Shift duration in minutes"
	iconSize={30}
/>
