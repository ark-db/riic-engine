<script lang="ts">
	import drones from '$lib/images/drones.webp';
	import { activeSave } from '$lib/stores';
	import NumberInput from './NumberInput.svelte';
</script>

<NumberInput
	min={0}
	max={999999}
	initial={$activeSave.data.drones}
	placeholder="Drones"
	onValidInput={(value) => ($activeSave.data.drones = value)}
	errorMsg="The specified drone capacity should be a number from 0 to 999999"
	iconSrc={drones}
	iconDesc="Drone capacity"
	iconSize={33}
/>
