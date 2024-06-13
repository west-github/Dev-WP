// import default_styles from '@repo/tailwind-config';

// const __b = require('@repo/modal-component');

/** @type {import('tailwindcss').Config} */
export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./lib/**/*.{html,js,svelte,ts}',
		'./routes/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {}
	},
	plugins: []
	// presets: [default_styles]
};
