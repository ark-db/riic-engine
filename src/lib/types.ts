export type FileData = {
	name: string;
	readonly modified: number;
	readonly created: number;
};

type Facility = {
	shifts: {
		char: string;
		start: number;
		end: number;
	}[];
	level: number;
};

type AtMostFive<T> = [T?, T?, T?, T?, T?];

export type SaveData = {
	layout: {
		cc: Facility;
		tp: AtMostFive<Facility>;
		fac: AtMostFive<Facility>;
		pp: AtMostFive<Facility>;
		rr: Facility;
		office: Facility;
		dorm: AtMostFive<Facility>;
	};
	chars: {
		char: string;
		tier: number;
	}[];
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
