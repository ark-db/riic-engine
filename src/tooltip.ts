const tooltipStyle = `
    position: absolute;
    border-radius: 0.25em;
    padding: 0.5em;
    background-color: black;
    color: var(--light);
`;

function getTextWidth(text: string): number {
	const canvas = document.createElement('canvas');
	const context = canvas.getContext('2d') ?? new CanvasRenderingContext2D();
	const metrics = context.measureText(text);
	return metrics.width;
}

export function tooltip(element: HTMLElement) {
	let div: HTMLDivElement;
	const title = element.title;
	const divWidth = getTextWidth(title) + 16;

	function calcPosition(event: MouseEvent): [number, number] {
		let left = event.pageX + 5;
		let top = event.pageY + 5;
		// check for page overflow
		if (left + divWidth + 4 > document.documentElement.clientWidth) {
			left = event.pageX - div.clientWidth + 5;
		}
		if (top + 32 > document.documentElement.clientHeight) {
			top = event.pageY - div.clientHeight + 5;
		}

		return [left, top];
	}

	function mouseOver(event: MouseEvent) {
		div = document.createElement('div');
		div.textContent = title;

		const [left, top] = calcPosition(event);
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
		const [left, top] = calcPosition(event);
		div.style.left = `${left}px`;
		div.style.top = `${top}px`;
	}
	function hide() {
		document.body.removeChild(div);
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
