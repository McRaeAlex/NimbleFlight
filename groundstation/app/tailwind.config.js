module.exports = {
  purge: ['index.html', 'main.js'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      gridTemplateRows: {
        'layout': '5vh repeat(3, 1fr)',
      }
    }
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
