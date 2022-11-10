export type FileData = {
	name: string;
	modified: number;
	created: number;
};

type Operator = string;

type Shift = {
	char: Operator;
	start: number;
	end: number;
};

type Facility3 = {
	shifts: Shift[];
	level: 1 | 2 | 3;
};

type Facility5 = {
	shifts: Shift[];
	level: 1 | 2 | 3 | 4 | 5;
};

type AtMostFive<T> = [T?, T?, T?, T?, T?];

type CharData = {
	char: Operator;
	elite: 0 | 1 | 2;
};

export type SaveData = {
	version: {
		major: number;
		minor: number;
	};
	layout: {
		cc: Facility5;
		tp: AtMostFive<Facility3>;
		fac: AtMostFive<Facility3>;
		pp: AtMostFive<Facility3>;
		rr: Facility3;
		office: Facility3;
		dorm: AtMostFive<Facility5>;
	};
	chars: CharData[];
};
