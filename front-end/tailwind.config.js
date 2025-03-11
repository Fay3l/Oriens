/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily:{
        raleway: ['Raleway', 'sans-serif'],
        'rock-salt': ['Rock Salt', 'cursive'],
        roboto: ['Roboto', 'sans-serif'],
      },
      colors:{
        orange:'#EE7213',
        orangedark:'#E16200',
        bluedark:'#15285C',
        blue:'#2F4B98',
        lightdark:'#1D1D1D',
      }
    },
  },
  plugins: [require('tailwindcss-primeui')],
}

