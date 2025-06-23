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
        jakarta: ['Plus Jakarta Sans', 'sans-serif'],
        shadows: ['Shadows Into Light Two', 'cursive'],
      },
      colors:{
        orange:'#EE7213',
        orangelight:'#F2A35D',
        orangedark:'#E16200',
        bluedark:'#15285C',
        blue:'#2F4B98',
        lightdark:'#1D1D1D',
      },
      animation: {
        'infinite-scroll': 'infinite-scroll 25s linear infinite',
      },
      keyframes: {
        'infinite-scroll': {
          from: { transform: 'translateX(0)' },
          to: { transform: 'translateX(-100%)' },
        }
      }
    },
  },
  plugins: [require('tailwindcss-primeui')],
}

