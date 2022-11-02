export type MaybePromise<T> = Promise<T> | undefined;

export type SaveData = {
	name: string;
	modified: number;
	created: number;
};
