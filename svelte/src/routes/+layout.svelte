<script lang="ts">
	import './app.pcss';
</script>

<slot />

<style lang="postcss">
	:global(.icon) {
		display: inline-block;
		position: relative;
		width: 1.25rem;
		height: 1.25rem;
		top: -1px;
		background-size: cover;
	}

	:global(body, html) {
		@apply size-full;
	}
</style>
