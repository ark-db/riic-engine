import { prefetch, goto } from '$app/navigation';
import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { SaveData, FileData } from '$lib/types';
import pencilClockIcon from '$lib/images/pencil-clock.svg';
import plusClockIcon from '$lib/images/plus-clock.svg';
import listIncreasingIcon from '$lib/images/list-increasing.svg';
import listDecreasingIcon from '$lib/images/list-decreasing.svg';

type SaveSortMode = 'modified' | 'created';
type SaveSortOrder = 'increasing' | 'decreasing';
type Save = {
	title: string;
	data: SaveData;
};

function createSaveList() {
	const { subscribe, set } = writable<FileData[]>();

	const load = () => invoke<FileData[]>('fetch_saves').then(set).catch(error.handle);

	return {
		subscribe,
		load,
		create: () => {
			invoke<void>('create_save').catch(error.handle);
			load();
		},
		export: (name: string) => {
			invoke<void>('export_save', { name }).catch(error.handle);
			load();
		},
		delete: (name: string) => {
			invoke<void>('delete_save', { name }).catch(error.handle);
			load();
		}
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
	const store = writable<SaveSortMode>('modified');
	const { subscribe, update } = store;

	const opposite = (mode: SaveSortMode): SaveSortMode =>
		mode === 'created' ? 'modified' : 'created';

	return {
		subscribe,
		src: () => (get(store) === 'created' ? plusClockIcon : pencilClockIcon),
		toggle: () => {
			update((mode) => opposite(mode));
			saveList.load();
		},
		nextDesc: () => `Sort by time ${opposite(get(store))}`
	};
}

function createSaveSortOrder() {
	const store = writable<SaveSortOrder>('increasing');
	const { subscribe, update } = store;

	return {
		subscribe,
		src: () => (get(store) === 'increasing' ? listIncreasingIcon : listDecreasingIcon),
		toggle: () => {
			update((order) => (order === 'increasing' ? 'decreasing' : 'increasing'));
			saveList.load();
		},
		nextDesc: () =>
			`Sort from ${get(store) === 'increasing' ? 'latest to earliest' : 'earliest to latest'}`
	};
}

function createActiveSave() {
	const { subscribe, set } = writable<Save>();

	async function loadSave(name: string) {
		prefetch('/editor');
		const data = await invoke<SaveData>('load_save', { name });
		set({
			title: name,
			data
		});
		goto('/editor');
	}

	return {
		subscribe,
		load: (name: string) => loadSave(name).catch(error.handle)
	};
}

export const saveList = createSaveList();
export const error = createError();
export const saveSortMode = createSaveSortMode();
export const saveSortOrder = createSaveSortOrder();
export const activeSave = createActiveSave();
