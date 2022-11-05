import { writable } from 'svelte/store';

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
