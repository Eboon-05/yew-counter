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
      'sans': ['"Source Sans Pro"', 'sans-serif'],
      'pacifico': ['Pacifico', 'cursive', 'sans-serif'],
      'itim': ['Itim', 'cursive', 'sans-serif'],
      'source-sans-pro': ['"Source Sans Pro"', 'sans-serif'],
    }
  },
  plugins: [],
  darkMode: 'class',
}

