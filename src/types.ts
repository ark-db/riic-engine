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

type Facility = {
	shifts: Shift[];
	level: 1 | 2 | 3 | 4 | 5;
};

type AtMostFive<T> = [T?, T?, T?, T?, T?];

type CharData = {
	char: Operator;
	tier: number;
};

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
	chars: CharData[];
};
