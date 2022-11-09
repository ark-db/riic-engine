export type MaybePromise<T> = Promise<T> | undefined;

export type FileData = {
	name: string;
	modified: number;
	created: number;
};
