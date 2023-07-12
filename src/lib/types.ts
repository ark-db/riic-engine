export type SaveTimeData = {
	name: string;
	readonly modified: number;
	readonly created: number;
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

export type Facility = NoShiftFacility & { shifts: (string | undefined)[] };

type BoostFacilityBase<P> = Facility & {
	boosts: (number | undefined)[];
	products: (P | undefined)[];
};

type TradingPost = BoostFacilityBase<TradingProduct>;
type Factory = BoostFacilityBase<FactoryProduct>;
export type BoostFacility = TradingPost | Factory;

export type TradingProduct = 'lmd' | 'orundum';
export type FactoryProduct = 'exp200' | 'exp400' | 'exp1000' | 'gold' | 'shard';
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
