/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "./src/*.rs",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {},
    fontFamily: {
      'pacifico': ['Pacifico', 'cursive', 'sans-serif'],
      'itim': ['Itim', 'cursive', 'sans-serif'],
    }
  },
  plugins: [],
  darkMode: 'class',
}

