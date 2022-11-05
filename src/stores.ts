import { writable } from 'svelte/store';

type SaveSortMode = 'modified' | 'created';
export const saveSortMode = writable<SaveSortMode>('modified');

type SaveSortOrder = 'increasing' | 'decreasing';
export const saveSortOrder = writable<SaveSortOrder>('increasing');
