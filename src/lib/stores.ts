import { goto } from '$app/navigation';
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { SaveData, FileData, ActiveSave } from '$lib/types';
import pencilClockIcon from '$lib/images/pencil-clock.svg';
import plusClockIcon from '$lib/images/plus-clock.svg';
import listIncreasingIcon from '$lib/images/list-increasing.svg';
import listDecreasingIcon from '$lib/images/list-decreasing.svg';

type SortMode = 'modified' | 'created';
type SaveSortMode = {
	mode: SortMode;
	nextDesc: `Sort by time ${SortMode}`;
};
type SaveSortOrder = {
	order: 'increasing' | 'decreasing';
	nextDesc: 'Sort from earliest to latest' | 'Sort from latest to earliest';
	direction: 1 | -1;
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

function createSaveSortOrder() {
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

export const saveList = createSaveList();
export const error = createError();
export const saveSortMode = createSaveSortMode();
export const saveSortOrder = createSaveSortOrder();
export const activeSave = createActiveSave();
