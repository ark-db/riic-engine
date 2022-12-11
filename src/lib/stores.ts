import { goto } from '$app/navigation';
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { SaveData, FileData } from '$lib/types';
import pencilClockIcon from '$lib/images/pencil-clock.svg';
import plusClockIcon from '$lib/images/plus-clock.svg';
import listIncreasingIcon from '$lib/images/list-increasing.svg';
import listDecreasingIcon from '$lib/images/list-decreasing.svg';

type SaveSortMode = 'modified' | 'created';
type SaveSortOrder = 'increasing' | 'decreasing';
type ActiveSave = {
	name: string;
	data: SaveData;
};

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

function createSaveSortMode() {
	const { subscribe, update } = writable<SaveSortMode>('modified');

	let mode: SaveSortMode;
	subscribe((value) => (mode = value));

	const opposite = (mode: SaveSortMode): SaveSortMode =>
		mode === 'created' ? 'modified' : 'created';

	return {
		subscribe,
		src: () => (mode === 'created' ? plusClockIcon : pencilClockIcon),
		toggle: () => {
			update((mode) => opposite(mode));
			saveList.load();
		},
		nextDesc: () => `Sort by time ${opposite(mode)}`
	};
}

function createSaveSortOrder() {
	const { subscribe, update } = writable<SaveSortOrder>('increasing');

	let order: SaveSortOrder;
	subscribe((value) => (order = value));

	return {
		subscribe,
		src: () => (order === 'increasing' ? listIncreasingIcon : listDecreasingIcon),
		toggle: () => {
			update((order) => (order === 'increasing' ? 'decreasing' : 'increasing'));
			saveList.load();
		},
		nextDesc: () =>
			`Sort from ${order === 'increasing' ? 'earliest to latest' : 'latest to earliest'}`
	};
}

function createActiveSave() {
	const { subscribe, set } = writable<ActiveSave>();

	subscribe((save) => invoke<void>('update_save', { save }).catch(error.handle));

	async function loadSave(name: string) {
		const data = await invoke<SaveData>('load_save', { name });
		set({ name, data });
		await invoke<void>('rename_window', { name });
		await goto('/editor/setup');
	}

	return {
		subscribe,
		set,
		load: (name: string) => loadSave(name).catch(error.handle)
	};
}

export const saveList = createSaveList();
export const error = createError();
export const saveSortMode = createSaveSortMode();
export const saveSortOrder = createSaveSortOrder();
export const activeSave = createActiveSave();
