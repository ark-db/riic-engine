import tippy, { followCursor, type Instance } from 'tippy.js/headless';

function render(instance: Instance) {
	const popper = document.createElement('div');

	popper.setAttribute(
		'style',
		`
        background-color: #000000;
        color: var(--light);
        border-radius: 0.5em;
        padding: 0.5em 0.6em;
    `
	);

	if (typeof instance.props.content === 'string') {
		popper.textContent = instance.props.content;
	}

	return { popper };
}

export function tooltip(element: HTMLElement | SVGElement) {
	const instance = tippy(element, {
		content: element.dataset.title,
		arrow: false,
		duration: 0,
		followCursor: true,
		plugins: [followCursor],
		render
	});

	return {
		destroy: instance.destroy
	};
}
