import charData from '$lib/data/chars.json';

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
