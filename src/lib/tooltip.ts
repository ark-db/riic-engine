import tippy, { followCursor } from 'tippy.js/headless';

export function tooltip(element: Element, content: string) {
	function render() {
		const popper = document.createElement('div');
		popper.className = 'tooltip-template';
		popper.textContent = content;

		return { popper };
	}

	const { destroy } = tippy(element, {
		arrow: false,
		followCursor: true,
		hideOnClick: false,
		plugins: [followCursor],
		render
	});

	// Destroy the tooltip when the element it is attached to is destroyed
	return {
		destroy
	};
}
