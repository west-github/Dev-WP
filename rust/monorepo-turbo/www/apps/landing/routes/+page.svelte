<script lang="ts">
	// import { media_query } from '@repo/common';
	import { PUBLIC_LOAD_TIME } from '$env/static/public';
	import { Modal } from '@repo/component';
	import { sleep } from '$lib';
	import '../src/app.pcss';

	// const media = media_query('(max-width: 600px)');
	let show_modal = $state(false);
</script>

<svelte:head>
	<title>Dapps - ETH</title>
</svelte:head>

<!-- <SvelteToast /> -->

{#await sleep(+PUBLIC_LOAD_TIME)}
	<div class="size-full center bg-red-400">
		<span class="loader"></span>
	</div>
{:then _}
	<Modal bind:show_modal></Modal>
{/await}

<!-- <style lang="postcss">
	:global(body, html) {
		@apply size-full;
	}

	:global(body) {
		@apply bg-gray-950 text-gray-400 relative;
	}

	:global(p) {
		@apply lg:text-base;
	}

	:global(span) {
		@apply text-xs;
	}

	:global(input) {
		@apply bg-transparent outline-none block w-full text-gray-400 p-2 pl-4 border text-base;
	}

	:global(button) {
		@apply active:scale-95 trans;
	}

	:global(:root) {
		--toastWidth: 'fit', --toastMinHeight: 'auto', --toastPadding: '5px';
	}

	:global(.loader) {
		@apply size-12 rounded-[50%] inline-block border-2 border-purple-500 relative animate-[rotation_1s_linear_infinite];
	}

	:global(.loader::after) {
		content: '';
		@apply absolute left-1 top-1 border-2 border-white size-3 rounded-[50%];
	}

	@keyframes rotation {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style> -->
