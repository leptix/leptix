const { blackA, mauve, violet, green, red } = require("@radix-ui/colors");

/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "selector",
	content: ["./src/**/*.{html,rs}"],
	theme: {
		extend: {
			colors: {
				...blackA,
				...mauve,
				...violet,
				...green,
				...red,
			},
		},
	},
	plugins: [],
};
