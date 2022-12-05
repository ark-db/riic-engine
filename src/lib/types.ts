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
	level: 1 | 2 | 3 | 4 | 5;
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
