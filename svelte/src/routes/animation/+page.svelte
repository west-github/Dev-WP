<section class="bg-purple-100"></section>
<section class="bg-purple-200"></section>
<div class="h-96 flex items-center justify-center">
	<div class="size-20 rounded bg-purple-400 animation"></div>
</div>

<style lang="postcss">
	section {
		@apply size-full;
	}

	.animation {
		/* animation: name duration timing-function delay iteration-count direction fill-mode; */
		animation: animate 1s;
		animation-iteration-count: 1;
	}

	@keyframes animate {
		from {
			transform: translateY(0%);
		}

		60% {
			transform: translateY(40%);
		}

		to {
			transform: translateY(0);
		}
	}
</style>
