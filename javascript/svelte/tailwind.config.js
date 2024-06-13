const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				sans: ["'Comfortaa Variable', sans-serif", ...defaultTheme.fontFamily.sans]
			}
		}
	},
	plugins: []
};
