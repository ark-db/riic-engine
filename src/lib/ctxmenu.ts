import tippy from 'tippy.js/headless';

export function context(element: Element, content: string) {
	function render() {
		const popper = document.createElement('div');
		popper.className = 'tooltip-template';
		popper.innerHTML = content;

		return { popper };
	}

	const { setProps, show, destroy } = tippy(element, {
		arrow: false,
		interactive: true,
		offset: [0, 0],
		placement: 'right-start',
		render,
		trigger: 'manual'
	});

	element.addEventListener('contextmenu', (event) => {
		if (event.isTrusted && event instanceof MouseEvent) {
			event.preventDefault();

			const { x, y } = event;

			setProps({
				getReferenceClientRect: () => ({
					width: 0,
					height: 0,
					top: y,
					bottom: y,
					left: x,
					right: x,
					x,
					y,
					toJSON: () => ({ x, y })
				})
			});

			show();
		}
	});

	element.addEventListener('keydown', (event) => {
		if (event.isTrusted && event instanceof KeyboardEvent && event.key === 'Enter') {
			show();
		}
	});

	// Destroy the context menu when the element it is attached to is destroyed
	return { destroy };
}
