<script lang="ts">
	import briefcase from '$lib/images/briefcase.svg';
	import { activeSave } from '$lib/stores';
	import NumberInput from './NumberInput.svelte';
</script>

<NumberInput
	desc="Number of shifts"
	min={0}
	max={999}
	initial={$activeSave.data.maxShift}
	placeholder=" "
	onValidInput={(value) => ($activeSave.data.maxShift = value)}
	errorMsg="The specified number of shifts should be a number from 0 to 999"
	iconSrc={briefcase}
	iconSize={36}
/>
