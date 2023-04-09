<script lang="ts">
	import briefcase from '$lib/images/ui/briefcase.svg';
	import { activeSave } from '$lib/stores';
	import NumberInput from './NumberInput.svelte';
</script>

<NumberInput
	desc="Number of shifts"
	min={0}
	max={99}
	initial={$activeSave.data.maxShift}
	placeholder=" "
	onValidInput={(value) => ($activeSave.data.maxShift = value)}
	errorMsg="The specified number of shifts should be a number from 0 to 99"
	iconSrc={briefcase}
	iconSize={36}
/>
