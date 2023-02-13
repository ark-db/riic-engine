// Helper function for running interaction logic on elements that are not interactive by default
export function interaction(fn: () => void, keys = ['Space', 'Enter']) {
	return (event: MouseEvent | KeyboardEvent) => {
		if (
			event.isTrusted &&
			((event instanceof MouseEvent && event.button === 0) ||
				(event instanceof KeyboardEvent && keys.includes(event.key)))
		) {
			fn();
		}
	};
}
