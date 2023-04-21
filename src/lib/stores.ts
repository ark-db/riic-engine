import { goto } from '$app/navigation';
import { navigating } from '$app/stores';
import { writable, derived } from 'svelte/store';
import { tweened } from 'svelte/motion';
import { cubicOut } from 'svelte/easing';
import { invoke } from '@tauri-apps/api/tauri';
import pencilClockIcon from '$lib/images/ui/pencil-clock.svg';
import plusClockIcon from '$lib/images/ui/plus-clock.svg';
import listIncreasingIcon from '$lib/images/ui/list-increasing.svg';
import listDecreasingIcon from '$lib/images/ui/list-decreasing.svg';
import maximizeIcon from '$lib/images/ui/maximize.svg';
import minimizeIcon from '$lib/images/ui/minimize.svg';
import type { SaveData, SaveTimeData } from '$lib/types';

// App-wide error store to display errors to users
function createError() {
	const { subscribe, set } = writable<string>('');

	function handle(err: unknown) {
		if (err instanceof Error || err instanceof ErrorEvent) {
			set(err.message);
		} else if (typeof err === 'string') {
			set(err);
		}
	}

	return {
		subscribe,
		clear: () => set(''),
		handle
	};
}

// State of the save export notice on the main menu
function createExportNotice() {
	const { subscribe, set } = writable<boolean>(false);

	return {
		subscribe,
		activate: () => set(true),
		deactivate: () => set(false)
	};
}

// Controls whether the save list on the main menu is sorted by time of creation or last edit
function createSaveSortMode() {
	type SortMode = 'modified' | 'created';
	type SaveSortMode = {
		mode: SortMode;
		nextDesc: `Sort by time ${SortMode}`;
	};

	const { subscribe, set } = writable<SaveSortMode>({
		mode: 'modified',
		nextDesc: 'Sort by time created'
	});

	let currentMode: SortMode;
	subscribe((value) => (currentMode = value.mode));

	return {
		subscribe,
		src: () => (currentMode === 'created' ? plusClockIcon : pencilClockIcon),
		toggle: () => {
			set({
				mode: currentMode === 'modified' ? 'created' : 'modified',
				nextDesc: currentMode === 'modified' ? 'Sort by time modified' : 'Sort by time created'
			});
			saveList.load();
		}
	};
}

// Controls whether the save list on the main menu is sorted oldest-to-newest or newest-to-oldest
function createSaveSortOrder() {
	type SortOrder = 1 | -1;
	type SaveSortOrder = {
		order: SortOrder;
		nextDesc: 'Sort from earliest to latest' | 'Sort from latest to earliest';
	};

	const { subscribe, set } = writable<SaveSortOrder>({
		order: 1,
		nextDesc: 'Sort from earliest to latest'
	});

	let currentOrder: SortOrder;
	subscribe((value) => (currentOrder = value.order));

	return {
		subscribe,
		src: () => (currentOrder === 1 ? listIncreasingIcon : listDecreasingIcon),
		toggle: () => {
			set({
				order: currentOrder === 1 ? -1 : 1,
				nextDesc:
					currentOrder === 1 ? 'Sort from latest to earliest' : 'Sort from earliest to latest'
			});
		}
	};
}

// The list of saves on the main menu. Interacting with the list (creating, deleting, etc.) will refresh it.
function createSaveList() {
	const saves = writable<SaveTimeData[]>();

	const { subscribe } = derived(
		[saves, saveSortMode, saveSortOrder],
		([$saves, { mode }, { order }]) =>
			$saves ? $saves.sort((prev, curr) => (prev[mode] - curr[mode]) * order) : []
	);

	const loadSaves = () => invoke<SaveTimeData[]>('fetch_saves').then(saves.set);

	return {
		subscribe,
		load: () => loadSaves().catch(error.handle),
		create: () => invoke<void>('create_save').then(loadSaves).catch(error.handle),
		export: (name: string) =>
			invoke<void>('export_save', { name })
				.then(exportNotice.activate)
				.then(loadSaves)
				.catch(error.handle),
		delete: (name: string) =>
			invoke<void>('delete_save', { name }).then(loadSaves).catch(error.handle)
	};
}

// Stores data of the currently-active save file
function createActiveSave() {
	const { subscribe, set } = writable<SaveData>();

	let saveName: string;

	/*
	The `subscribe` method on `activeSave` runs whenever a non-root page loads.
	When this happens, the `update_save` command should NOT be invoked: if it did,
	the last-modified time of a save would be updated without any edits made!
	The `navigating` store is used to determine if a page is loading. 
	*/
	let loading = true;
	navigating.subscribe((value) => (loading = !!value));
	subscribe((save) => {
		if (save && !loading) invoke<void>('update_save', { name: saveName, save }).catch(error.handle);
	});

	async function loadSave(name: string) {
		const data = await invoke<SaveData>('get_save', { name });
		loading = true;
		set(data);
		saveName = name;
		await goto('/editor/setup');
		await invoke<void>('rename_window', { name });
	}

	return {
		subscribe,
		set,
		load: (name: string) => loadSave(name).catch(error.handle)
	};
}

// The zoom values in the save editor. Zoom values for the X and Y axes are independently controlled.
function createZoomControls() {
	const minFactor = 0;
	const maxFactor = 3;

	const initTweened = () =>
		tweened(0, {
			duration: 300,
			easing: cubicOut
		});

	const xFactor = initTweened();
	const yFactor = initTweened();

	function clamp(value: number): number {
		return Math.min(maxFactor, Math.max(minFactor, value));
	}

	return {
		xFactor,
		yFactor,
		minFactor,
		maxFactor,
		changeX: (value: number) => {
			xFactor.update((old) => clamp(old + value));
		},
		changeY: (value: number) => {
			yFactor.update((old) => clamp(old + value));
		}
	};
}

// Shortcut (minimize + maximize) store for the save editor zoom controls
function createZoomShortcut() {
	type ShortcutMode = 'min' | 'max';
	type ShortcutDetails = {
		src: string;
		desc: string;
		run: () => void;
	};

	const { xFactor, yFactor, minFactor, maxFactor } = zoomControls;

	const mode = writable<ShortcutMode>('max');

	derived([xFactor, yFactor], (val) => val).subscribe(([x, y]) => {
		if (x === minFactor && y === minFactor) mode.set('max');
		if (x === maxFactor && y === maxFactor) mode.set('min');
	});

	function setMin() {
		xFactor.set(minFactor);
		yFactor.set(minFactor);
	}
	function setMax() {
		xFactor.set(maxFactor);
		yFactor.set(maxFactor);
	}

	return derived<typeof mode, ShortcutDetails>(mode, (mode) =>
		mode === 'max'
			? {
					src: maximizeIcon,
					desc: 'Zoom to maximum',
					run: setMax
			  }
			: {
					src: minimizeIcon,
					desc: 'Zoom to minimum',
					run: setMin
			  }
	);
}

function createZoomScales() {
	const { xFactor, yFactor } = zoomControls;

	return {
		xScale: derived(xFactor, ($x) => 2 ** $x),
		yScale: derived(yFactor, ($y) => 2 ** $y)
	};
}

export const error = createError();
export const exportNotice = createExportNotice();
export const saveSortMode = createSaveSortMode();
export const saveSortOrder = createSaveSortOrder();
export const saveList = createSaveList();
export const activeSave = createActiveSave();
export const zoomControls = createZoomControls();
export const zoomShortcut = createZoomShortcut();
export const zoomScales = createZoomScales();
