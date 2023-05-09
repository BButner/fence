/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./plugins/**/*.{js,ts}",
    "./nuxt.config.{js,ts}",
    "./app.vue",
  ],
  theme: {
    extend: {
      colors: {
        'tango': {
          '50': '#fef8ee',
          '100': '#fdefd7',
          '200': '#fbdaad',
          '300': '#f7c07a',
          '400': '#f39b44',
          '500': '#f07f21', // default
          '600': '#e16415',
          '700': '#bb4b13',
          '800': '#953d17',
          '900': '#783316',
          '950': '#411709',
        },
        'white-rock': {
          '50': '#f9f7f3',
          '100': '#f0ece0', // default
          '200': '#e3dac5',
          '300': '#d0c1a1',
          '400': '#bda47a',
          '500': '#af8e60',
          '600': '#a27d54',
          '700': '#876547',
          '800': '#6e533e',
          '900': '#594435',
          '950': '#2f231b',
        },
        'fiord': {
          '50': '#f6f7f9',
          '100': '#eceef2',
          '200': '#d5d9e2',
          '300': '#b0b7c9',
          '400': '#8691aa',
          '500': '#677390',
          '600': '#525c77',
          '700': '#474f67', // default
          '800': '#3a4152',
          '900': '#343846',
          '950': '#22252f',
        },
        'mako': {
          '50': '#f4f4f7',
          '100': '#e4e6e9',
          '200': '#cbcfd6',
          '300': '#a8afb8',
          '400': '#7c8594',
          '500': '#616979',
          '600': '#535967',
          '700': '#474b56', // default
          '800': '#3f424b',
          '900': '#383a41',
          '950': '#232529',
        },
        'twilight-blue': {
          '50': '#f9feff', // default
          '100': '#e0f9fe',
          '200': '#baf6fd',
          '300': '#7df1fc',
          '400': '#38eaf8',
          '500': '#0ed8e9',
          '600': '#02b2c7',
          '700': '#038ea1',
          '800': '#077685',
          '900': '#0c616e',
          '950': '#083e49',
        },
      }
    },
  },
  plugins: [],
}

