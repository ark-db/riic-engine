import charData from '$lib/data/chars.json';
import facilityData from '$lib/data/facilities.json';
import skillData from '$lib/data/skills.json';
import termsData from '$lib/data/terms.json';
import textColorsData from '$lib/data/text-colors.json';
import type { FacilityName } from '$lib/types';

export const chars = charData as Readonly<
	{
		charId: string;
		name: string;
		rarity: number;
		skills: {
			elite: number;
			level: number;
			skillId: string;
		}[];
	}[]
>;

export const facilities = facilityData as Record<
	FacilityName,
	Readonly<{
		name: string;
		power: number[];
		capacity: number[];
	}>
>;

export const skills = skillData as Record<
	string,
	Readonly<{
		name: string;
		desc: string;
		iconId: string;
	}>
>;

export const terms = termsData as Record<string, string>;

export const textColors = textColorsData as Record<string, string>;
