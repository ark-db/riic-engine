export type FileData = {
	name: string;
	readonly modified: number;
	readonly created: number;
};

export type Facility = {
	shifts: {
		char: string;
		start: number;
		end: number;
	}[];
	level: number;
};

export type SaveData = {
	layout: {
		cc: Facility;
		tp: Facility[];
		fac: Facility[];
		pp: Facility[];
		workshop: Facility;
		rr: Facility;
		office: Facility;
		train: Facility;
		dorm: Facility[];
	};
	chars: {
		char: string;
		tier: number;
	}[];
	drones: number;
};

export type ActiveSave = {
	name: string;
	data: SaveData;
};

export type FacilityName =
	| 'control'
	| 'dormitory'
	| 'hire'
	| 'manufacture'
	| 'meeting'
	| 'power'
	| 'trading'
	| 'training'
	| 'workshop';
