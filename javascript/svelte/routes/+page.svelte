<script lang="ts">
	import Foo from './foo.svelte';
	import { spring, tweened } from 'svelte/motion';
	import Input from './input.svelte';
	import { onMount } from 'svelte';
	import Count from './count.svelte';

	const counts = [Count, Count];

	let count = 1;

	$: area = count * 30;

	let _tweened = tweened(20);

	let _spring = spring(2);

	let action = true;

	function actionHandler(node: HTMLParagraphElement) {
		node.textContent += ' Some Value is added';

		return {
			destroy: () => console.log('Unmounted')
		};
	}

	function btnHandler(_: MouseEvent) {
		count++;

		const closure = (args: number) => args + 30;
		_tweened.update(closure);
		_spring.update(closure);

		action = !action;
	}
</script>

<button on:click={btnHandler}> Click Me </button>

{#if action}
	<p use:actionHandler>This element is an action with value: {action}</p>
{/if}

<p use:actionHandler>This element is an action with value: {action}</p>

{#each counts as count}
	<svelte:component this={count} count={10} />
{/each}

<p style:color|important="red">Count: {count}</p>
<p>Area: {area}</p>

<Foo class="West" />

<p>Tweened: {$_tweened}</p>

<p>Spring: {$_spring}</p>

<Input on:input={(event) => console.log(event.target)} />

<div class="flex items-center justify-center w-40 gap-3">
	<span>Value</span> <span class="image icon"></span>
</div>

<div class="animate-width text-black font-bold">
	<input type="checkbox" id="checkbox" class="peer size-5 inline-block absolute top-2 right-4" />
	<label
		for="checkbox"
		class="bg-white rounded peer-checked:rounded-b-none flex items-center pl-5 cursor-pointer select-none text-sm h-10"
	>
		CheckBox Label
	</label>

	<div class="animate-test bg-white hidden peer-checked:block -mt-1">
		<div class="center gradient rounded p-4"></div>
	</div>
</div>

<div
	class="grid grid-cols-3 gap-3 grid-rows-2 border p-4 w-fit mt-10 rounded-lg border-purple-400 justify-items-center"
>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
	<div class="size-52 bg-gradient-to-br from-violet-900 from-30% to-fuchsia-600 rounded"></div>
</div>

<div class="grid grid-flow-col auto-cols-max m-10 gap-10">
	<div class="size-96 bg-purple-600 rounded center">01</div>
	<div class="size-96 bg-purple-600 rounded center">02</div>
	<div class="size-96 bg-purple-600 rounded center">03</div>
</div>

<div class="p-4 w-[800px] border mt-10">
	<div class="w-52 h-20 bg-purple-800 rounded float-left mr-4 mb-1"></div>

	<div class="h-96 bg-red-50"></div>
</div>

<style lang="postcss">
	.image {
		background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" fill="antiquewhite" viewBox="0 0 24 24" stroke-width="1.5" stroke="antiquewhite" class="w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75" /></svg>');
	}

	button {
		@apply bg-violet-900 text-sm py-2 px-10 drop-shadow-xl w-fit rounded;
	}

	.animate-test {
		@apply rounded-b  p-4 cursor-pointer;
		/* animation: name duration timing-function delay iteration-count direction fill-mode; */
		animation: test 200ms;
		animation-iteration-count: 1;
		animation-fill-mode: forwards;
	}

	.animate-width {
		@apply text-center  rounded transition-all duration-300 ease-in-out fixed left-1/2 top-10 -translate-x-1/2 w-96;
	}

	body {
		@apply !overflow-hidden;
	}

	input[type='checkbox']:checked::after {
		@apply block;
	}

	input[type='checkbox']::before {
		content: '';
		@apply absolute size-6 border-2 border-purple-400 bg-purple-50;
	}

	input[type='checkbox']::after {
		content: '✔️';
		@apply absolute text-xs top-[4px] left-[3px] hidden;
		animation: test 200ms;
		animation-iteration-count: 1;
		animation-fill-mode: forwards;
	}

	@keyframes test {
		from {
			transform: translateY(-10%);
		}
	}
</style>
