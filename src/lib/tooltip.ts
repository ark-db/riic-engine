import tippy, { followCursor, type Instance } from 'tippy.js/headless';

function render(instance: Instance) {
	const popper = document.createElement('div');

	popper.className = 'tooltip-template';

	if (typeof instance.props.content === 'string') {
		popper.textContent = instance.props.content;
	}

	return { popper };
}

export function tooltip(element: Element, content: string) {
	const { destroy } = tippy(element, {
		arrow: false,
		content,
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
