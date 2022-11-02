import { writable } from 'svelte/store';

type SaveSortMode = 'modified' | 'created';

export const saveSortMode = writable('modified' as SaveSortMode);
