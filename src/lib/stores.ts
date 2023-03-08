import { goto } from '$app/navigation';
import { writable, derived } from 'svelte/store';
import { tweened } from 'svelte/motion';
import { cubicOut } from 'svelte/easing';
import { invoke } from '@tauri-apps/api/tauri';
import pencilClockIcon from '$lib/images/pencil-clock.svg';
import plusClockIcon from '$lib/images/plus-clock.svg';
import listIncreasingIcon from '$lib/images/list-increasing.svg';
import listDecreasingIcon from '$lib/images/list-decreasing.svg';
import maximizeIcon from '$lib/images/maximize.svg';
import minimizeIcon from '$lib/images/minimize.svg';
import type { SaveData, FileData, ActiveSave, Facility, BoostFacility } from '$lib/types';

// The list of saves on the main menu. Interacting with the list (creating, deleting, etc.) will refresh it.
function createSaveList() {
	const { subscribe, set } = writable<FileData[]>();

	const loadSaves = () => invoke<FileData[]>('fetch_saves').then(set);

	return {
		subscribe,
		load: () => loadSaves().catch(error.handle),
		create: () => invoke<void>('create_save').then(loadSaves).catch(error.handle),
		export: (name: string) =>
			invoke<void>('export_save', { name }).then(loadSaves).catch(error.handle),
		delete: (name: string) =>
			invoke<void>('delete_save', { name }).then(loadSaves).catch(error.handle)
	};
}

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

// Controls whether the save list on the main menu is sorted by time of creation or last edit
function createSaveSortMode() {
	type SortMode = 'modified' | 'created';
	type SaveSortMode = {
		mode: SortMode;
		nextDesc: `Sort by time ${SortMode}`;
	};

	const { subscribe, update } = writable<SaveSortMode>({
		mode: 'modified',
		nextDesc: 'Sort by time created'
	});

	let data: SaveSortMode;
	subscribe((value) => (data = value));

	return {
		subscribe,
		src: () => (data.mode === 'created' ? plusClockIcon : pencilClockIcon),
		toggle: () => {
			update((data) => ({
				mode: data.mode === 'modified' ? 'created' : 'modified',
				nextDesc: data.mode === 'modified' ? 'Sort by time modified' : 'Sort by time created'
			}));
			saveList.load();
		}
	};
}

// Controls whether the save list on the main menu is sorted oldest-to-newest or newest-to-oldest
function createSaveSortOrder() {
	type SaveSortOrder = {
		order: 'increasing' | 'decreasing';
		nextDesc: 'Sort from earliest to latest' | 'Sort from latest to earliest';
		direction: 1 | -1;
	};

	const { subscribe, update } = writable<SaveSortOrder>({
		order: 'increasing',
		nextDesc: 'Sort from earliest to latest',
		direction: 1
	});

	let data: SaveSortOrder;
	subscribe((value) => (data = value));

	return {
		subscribe,
		src: () => (data.order === 'increasing' ? listIncreasingIcon : listDecreasingIcon),
		toggle: () => {
			update((data) => ({
				order: data.order === 'increasing' ? 'decreasing' : 'increasing',
				nextDesc:
					data.order === 'increasing'
						? 'Sort from latest to earliest'
						: 'Sort from earliest to latest',
				direction: data.direction === 1 ? -1 : 1
			}));
			saveList.load();
		}
	};
}

// Stores data of the currently-active save file
function createActiveSave() {
	const { subscribe, set } = writable<ActiveSave>();

	async function loadSave(name: string) {
		const data = await invoke<SaveData>('load_save', { name });
		set({ name, data });
		await goto('/editor/setup');
	}

	return {
		subscribe,
		set,
		load: (name: string) => loadSave(name).catch(error.handle)
	};
}

// Calculates the last column number in a base layout
function calculateLastColumnNumber(save: ActiveSave): number {
	// When there are no shifts specified in the save, the result is -Infinity, so a "floor" value is used
	return (
		Math.max(
			10,
			...Object.values(save.data.layout)
				.flat()
				.map((facility) =>
					Math.max(
						...((facility as Facility | BoostFacility)?.shifts?.map(({ end }) => end) ?? []),
						...((facility as BoostFacility)?.boosts?.map(({ col }) => col) ?? [])
					)
				)
		) + 2
	);
}

function createZoomControls() {
	const minZoom = 1;
	const maxZoom = 4;

	const initTweened = () =>
		tweened(1, {
			duration: 300,
			easing: cubicOut
		});

	const x = initTweened();
	const y = initTweened();

	function clamp(value: number): number {
		return Math.min(maxZoom, Math.max(minZoom, value));
	}

	return {
		x,
		y,
		min: minZoom,
		max: maxZoom,
		changeX: (value: number) => {
			x.update((old) => clamp(old + value));
		},
		changeY: (value: number) => {
			y.update((old) => clamp(old + value));
		}
	};
}

function createZoomShortcut() {
	type ShortcutMode = 'min' | 'max';
	type ShortcutDetails = {
		src: string;
		desc: string;
		run: () => void;
	};

	const { x, y, min, max } = zoomControls;

	const mode = writable<ShortcutMode>('max');

	derived([x, y], (val) => val).subscribe(([x, y]) => {
		if (x === min && y === min) mode.set('max');
		if (x === max && y === max) mode.set('min');
	});

	function setMin() {
		x.set(min);
		y.set(min);
	}
	function setMax() {
		x.set(max);
		y.set(max);
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

export const saveList = createSaveList();
export const error = createError();
export const saveSortMode = createSaveSortMode();
export const saveSortOrder = createSaveSortOrder();
export const activeSave = createActiveSave();
export const lastColumnNumber = derived(activeSave, calculateLastColumnNumber);
export const zoomControls = createZoomControls();
export const zoomShortcut = createZoomShortcut();
