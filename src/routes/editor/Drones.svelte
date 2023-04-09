<script lang="ts">
	import drones from '$lib/images/ui/drones.webp';
	import { activeSave } from '$lib/stores';
	import NumberInput from './NumberInput.svelte';
</script>

<NumberInput
	desc="Drone capacity"
	min={0}
	max={999999}
	initial={$activeSave.drones}
	placeholder="Drones"
	onValidInput={(value) => ($activeSave.drones = value)}
	errorMsg="The specified drone capacity should be a number from 0 to 999999"
	iconSrc={drones}
	iconSize={33}
/>
