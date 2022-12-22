<div class="container">
	<slot />
</div>

<style>
	.container {
		position: relative;
		border-radius: var(--radius);
		padding: var(--weight);
		background-color: var(--bg-color);
		background-clip: content-box;
	}
	.container:after,
	.container:before {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		content: '';
		border-radius: inherit;
	}
	.container:after {
		background: inherit;
		z-index: -1;
		transition: opacity 0.25s;
	}
	.container:before {
		background: linear-gradient(to right, var(--blue-strong), var(--blue-mild));
		z-index: -2;
	}
	.container:is(:hover, :focus-within):after {
		opacity: 0;
	}
</style>
