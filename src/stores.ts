import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { SaveData } from '@types';

type SaveSortMode = 'modified' | 'created';

function createSaveSortMode() {
	const store = writable<SaveSortMode>('modified');
	const { subscribe, update } = store;

	const opposite = (mode: SaveSortMode): SaveSortMode =>
		mode === 'created' ? 'modified' : 'created';

	return {
		subscribe,
		toggle: () => update((mode) => opposite(mode)),
		nextDesc: () => `Sort by time ${opposite(get(store))}`
	};
}

export const saveSortMode = createSaveSortMode();

type SaveSortOrder = 'increasing' | 'decreasing';

function createSaveSortOrder() {
	const store = writable<SaveSortOrder>('increasing');
	const { subscribe, update } = store;

	return {
		subscribe,
		toggle: () => update((order) => (order === 'increasing' ? 'decreasing' : 'increasing')),
		nextDesc: () =>
			`Sort from ${get(store) === 'increasing' ? 'latest to earliest' : 'earliest to latest'}`
	};
}

export const saveSortOrder = createSaveSortOrder();

type Save = {
	title: string;
	data: SaveData;
};

function createActiveSave() {
	const { subscribe, set } = writable<Save>();

	async function load(name: string) {
		const data = await invoke<SaveData>('load_save', { name });
		set({
			title: name,
			data
		});
	}

	return {
		subscribe,
		load
	};
}

export const activeSave = createActiveSave();
