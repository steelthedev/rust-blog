/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        'primary': '#F5F5DC',
        'main-text':'#111', // Replace with your desired color code
      },
      fontFamily:{
        main:[
          'Sofia Sans Semi Condensed'
        ]
      }
    },
  },
  plugins: [],
}