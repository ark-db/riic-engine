export function interaction(fn: () => void, keys = ['Space', 'Enter']) {
	return (event: MouseEvent | KeyboardEvent) => {
		if (
			(event instanceof MouseEvent && event.button === 0) ||
			(event instanceof KeyboardEvent && keys.includes(event.key))
		) {
			fn();
		}
	};
}
