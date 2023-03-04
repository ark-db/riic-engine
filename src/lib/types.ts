export type FileData = {
	name: string;
	readonly modified: number;
	readonly created: number;
};

export type ActiveSave = {
	name: string;
	data: SaveData;
};

export type SaveData = {
	layout: {
		cc: Facility;
		tp: BoostFacility[];
		fac: BoostFacility[];
		pp: Facility[];
		workshop: NoShiftFacility;
		rr: Facility;
		office: Facility;
		train: NoShiftFacility;
		dorm: Facility[];
	};
	chars: {
		char: string;
		tier: number;
	}[];
	drones: number;
};

export type Facility = {
	level: number;
	shifts: Shift[];
};

export type BoostFacility = {
	level: number;
	shifts: Shift[];
	boosts: {
		drones: number;
		col: number;
	}[];
};

type NoShiftFacility = {
	level: number;
};

type Shift = {
	char: string;
	start: number;
	end: number;
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
