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

	function calcPosition(event: MouseEvent): [number, number] {
		let left = event.pageX + 5;
		let top = event.pageY + 5;
		// check for page overflow
		if (left + div.clientWidth > document.documentElement.clientWidth) {
			left = event.pageX - div.clientWidth + 5;
		}
		if (top + div.clientHeight > document.documentElement.clientHeight) {
			top = event.pageY - div.clientHeight + 5;
		}

		return [left, top];
	}

	function mouseOver(event: MouseEvent) {
		// remove the `title` attribute to prevent showing the default browser tooltip
		title = element.getAttribute('title');
		element.removeAttribute('title');

		div = document.createElement('div');
		div.textContent = title;

		let [left, top] = calcPosition(event);
		div.setAttribute(
			'style',
			`
            ${tooltipStyle}
			left: ${left}px;
			top: ${top}px;
        `
		);

		document.body.appendChild(div);
	}
	function mouseMove(event: MouseEvent) {
		let [left, top] = calcPosition(event);
		div.style.left = `${left}px`;
		div.style.top = `${top}px`;
	}
	function hide() {
		document.body.removeChild(div);
		// restore the `title` attribute
		element.setAttribute('title', title ?? '');
	}

	element.addEventListener('mouseover', mouseOver);
	element.addEventListener('mouseleave', hide);
	element.addEventListener('mousemove', mouseMove);

	return {
		destroy() {
			element.removeEventListener('mouseover', mouseOver);
			element.removeEventListener('mouseleave', hide);
			element.removeEventListener('mousemove', mouseMove);
			hide();
		}
	};
}
