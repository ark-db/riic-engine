import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { SaveData } from '@types';

type SaveSortMode = 'modified' | 'created';
export const saveSortMode = writable<SaveSortMode>('modified');

type SaveSortOrder = 'increasing' | 'decreasing';

function createSaveSortOrder() {
	const { subscribe, update } = writable<SaveSortOrder>('increasing');

	return {
		subscribe,
		toggle: () => update((order) => (order === 'increasing' ? 'decreasing' : 'increasing')),
		nextDesc: (value: SaveSortOrder) =>
			value === 'increasing' ? 'latest to earliest' : 'earliest to latest'
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
