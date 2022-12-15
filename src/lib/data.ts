import charData from '$lib/data/chars.json';
import facilityData from '$lib/data/facilities.json';
import type { FacilityName } from '$lib/types';

export const chars = charData as {
	charId: string;
	name: string;
	rarity: number;
	skills: {
		elite: number;
		level: number;
		skillId: string;
	}[];
}[];

export const facilities = facilityData as Record<
	FacilityName,
	{
		name: string;
		power: readonly number[];
		capacity: readonly number[];
	}
>;
