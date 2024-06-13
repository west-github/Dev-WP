<script lang="ts">
	import '../src/app.pcss';

	const links = ['/', '/table', '/animation'];

	function uc_first(value: string): string {
		return value.slice(1, 2).toUpperCase() + value.slice(2);
	}
</script>

<div class="p-4 flex items-center justify-center">
	<ul class="space-y-3">
		{#each links as link}
			<li class="block w-fit px-4">
				<a href={link}>
					{link.slice(1) == '' ? 'Home' : uc_first(link)}
				</a>
			</li>
		{/each}
	</ul>
</div>
<slot />

<style lang="postcss">
	a {
		@apply text-xs text-red-400 font-semibold;
	}
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
