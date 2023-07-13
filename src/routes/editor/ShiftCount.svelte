<script lang="ts">
	import NumberInput from '$lib/components/NumberInput.svelte';
	import briefcase from '$lib/images/ui/briefcase.svg';
	import { activeSave } from '$lib/stores';
</script>

<NumberInput
	desc="Number of shifts"
	min={0}
	max={99}
	initial={$activeSave.maxShift}
	placeholder=" "
	onValidInput={(value) => ($activeSave.maxShift = value)}
	errorMsg="The specified number of shifts should be a number from 0 to 99"
	iconSrc={briefcase}
	iconSize={36}
/>
