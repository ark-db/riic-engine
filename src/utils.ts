export function interaction(fn: () => void) {
	return (event: MouseEvent | KeyboardEvent) => {
		if (
			(event instanceof MouseEvent && event.button === 0) ||
			(event instanceof KeyboardEvent && (event.key === 'Space' || event.key === 'Enter'))
		) {
			fn();
		}
	};
}
