<script lang="ts">
	import Button from '$lib/components/Button.svelte';

	export let onRename: () => void;
	export let onExport: () => void;
	export let onDelete: () => void;
</script>

<div>
	<Button desc="Rename save" onClick={onRename}>
		<svg
			class="edit-title"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 512 512"
			width="24"
			height="24"
		>
			<path
				d="M43 16C28.0656 16 16 28.0656 16 43C16 57.9344 28.0656 70 43 70L151 70L151 367C151 381.934 163.066 394 178 394C192.934 394 205 381.934 205 367L205 70L313 70C327.934 70 340 57.9344 340 43C340 28.0656 327.934 16 313 16L178 16L43 16Z"
				fill="#9a9696"
			/>
			<path
				d="M421.544 249.384L397.318 273.609L462.388 338.675L486.615 314.45C499.128 301.938 499.128 281.667 486.615 269.154L466.893 249.384C454.38 236.872 434.108 236.872 421.594 249.384L421.544 249.384ZM386.005 284.92L269.328 401.638C264.123 406.843 260.319 413.3 258.216 420.357L240.497 480.568C239.246 484.822 240.397 489.377 243.5 492.48C246.604 495.583 251.159 496.734 255.363 495.533L315.579 477.815C322.636 475.713 329.093 471.909 334.299 466.704L451.076 349.986C451.076 349.986 386.005 284.92 386.005 284.92Z"
				fill="#9a9696"
			/>
		</svg>
	</Button>
	<Button desc="Export save" onClick={onExport}>
		<svg
			class="export"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 640 512"
			width="26"
			height="26"
		>
			<path
				fill="#9a9696"
				d="M32 64C32 28.7 60.7 0 96 0H256V128c0 17.7 14.3 32 32 32H416V288H248c-13.3 0-24 10.7-24 24s10.7 24 24 24H416V448c0 35.3-28.7 64-64 64H96c-35.3 0-64-28.7-64-64V64zM416 336V288H526.1l-39-39c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l80 80c9.4 9.4 9.4 24.6 0 33.9l-80 80c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l39-39H416zm0-208H288V0L416 128z"
			/>
		</svg>
	</Button>
	<Button desc="Delete save" onClick={onDelete}>
		<svg
			class="trash"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 448 512"
			width="21"
			height="21"
		>
			<path
				fill="#9a9696"
				d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
			/>
		</svg>
	</Button>
</div>

<style>
	div {
		margin-right: 5px;
		display: flex;
		align-items: center;
		column-gap: 15px;
	}
	path {
		transition: fill 0.3s;
	}
	button:is(:hover, :focus-within) > .edit-title path,
	button:is(:hover, :focus-within) > .export path {
		fill: var(--light);
	}
	button:is(:hover, :focus-within) > .trash path {
		fill: var(--salmon-strong);
	}
</style>
