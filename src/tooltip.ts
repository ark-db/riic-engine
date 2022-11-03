const tooltipStyle = `
    position: absolute;
    border-radius: 0.25em;
    padding: 0.5em;
    background-color: black;
    color: var(--light);
`;

export function tooltip(element: HTMLElement) {
	let div: HTMLDivElement;
	let title: string | null;
	let position: DOMRect;
	function focus() {
		position = element.getBoundingClientRect();
		// remove the `title` attribute to prevent showing the default browser tooltip
		title = element.getAttribute('title');
		element.removeAttribute('title');

		div = document.createElement('div');
		div.textContent = title;
		div.setAttribute(
			'style',
			`
            ${tooltipStyle}
            top: ${position.top + 10}px;
            left: ${position.left + 5}px;
        `
		);
		document.body.appendChild(div);
	}
	function mouseOver(event: MouseEvent) {
		// remove the `title` attribute to prevent showing the default browser tooltip
		title = element.getAttribute('title');
		element.removeAttribute('title');

		div = document.createElement('div');
		div.textContent = title;
		div.setAttribute(
			'style',
			`
            ${tooltipStyle}
            top: ${event.pageX + 5}px;
            left: ${event.pageY + 5}px;
        `
		);
		document.body.appendChild(div);
	}
	function mouseMove(event: MouseEvent) {
		div.style.left = `${event.pageX + 5}px`;
		div.style.top = `${event.pageY + 5}px`;
	}
	function hide() {
		document.body.removeChild(div);
		// restore the `title` attribute
		element.setAttribute('title', title ?? '');
	}

	element.addEventListener('focus', focus);
	element.addEventListener('blur', hide);
	element.addEventListener('mouseover', mouseOver);
	element.addEventListener('mouseleave', hide);
	element.addEventListener('mousemove', mouseMove);

	return {
		destroy() {
			element.removeEventListener('focus', focus);
			element.removeEventListener('blur', hide);
			element.removeEventListener('mouseover', mouseOver);
			element.removeEventListener('mouseleave', hide);
			element.removeEventListener('mousemove', mouseMove);
		}
	};
}
