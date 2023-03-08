<div>
	<slot />
</div>

<style>
	div {
		position: relative;
		border-radius: var(--radius);
		padding: var(--weight);
		background-color: var(--bg-color);
		background-clip: content-box;
	}
	div:after,
	div:before {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		content: '';
		border-radius: inherit;
	}
	div:after {
		background: inherit;
		z-index: -1;
		transition: opacity 0.25s;
	}
	div:before {
		background: linear-gradient(to right, var(--blue-strong), var(--blue-mild));
		z-index: -2;
	}
	div:is(:hover, :focus-within):after {
		opacity: 0;
	}
</style>
