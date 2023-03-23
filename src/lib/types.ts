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
		tp: TradingPost[];
		fac: Factory[];
		pp: Facility[];
		workshop: NoShiftFacility;
		rr: Facility;
		office: Facility;
		train: NoShiftFacility;
		dorm: FixedLengthArray<[Facility, Facility, Facility, Facility]>;
	};
	chars: {
		char: string;
		tier: number;
	}[];
	drones: number;
	maxShift: number;
	interval: number;
};

type NoShiftFacility = {
	level: number;
};

export type Facility = NoShiftFacility & { shifts: Shift[] };

type BoostFacilityBase<P> = Facility & {
	boosts: {
		drones: number;
		col: number;
	}[];
	products: {
		kind: P;
		start: number;
		end: number;
	}[];
};

type TradingPost = BoostFacilityBase<TradingProduct>;
type Factory = BoostFacilityBase<FactoryProduct>;
export type BoostFacility = TradingPost | Factory;

type Shift = {
	char: string;
	start: number;
	end: number;
};

type TradingProduct = 'lmd' | 'orundum';
type FactoryProduct = 'exp200' | 'exp400' | 'exp1000' | 'gold' | 'shard';
export type Product = TradingProduct | FactoryProduct;

type ArrayLengthMutationKeys = 'splice' | 'push' | 'pop' | 'shift' | 'unshift' | number;
type ArrayItems<T extends unknown[]> = T extends Array<infer TItems> ? TItems : never;
type FixedLengthArray<T extends unknown[]> = Pick<T, Exclude<keyof T, ArrayLengthMutationKeys>> & {
	[Symbol.iterator]: () => IterableIterator<ArrayItems<T>>;
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
