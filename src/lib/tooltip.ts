import tippy, { followCursor, type Instance } from 'tippy.js/headless';

function render(instance: Instance) {
	const popper = document.createElement('div');

	popper.setAttribute(
		'style',
		`
		box-shadow: 0 4px 6px rgba(0 0 0 / 0.2);
		border: 1px solid var(--darkish);
        border-radius: 0.5em;
        padding: 0.5em 0.6em;
		background-color: #000000;
        color: var(--light);
    `
	);

	if (typeof instance.props.content === 'string') {
		popper.textContent = instance.props.content;
	}

	return { popper };
}

export function tooltip(element: Element, content: string) {
	const { destroy } = tippy(element, {
		content,
		arrow: false,
		hideOnClick: false,
		followCursor: true,
		plugins: [followCursor],
		render
	});

	// Destroy the tooltip when the element it is attached to is destroyed
	return {
		destroy
	};
}
