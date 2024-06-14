/** @type {import('tailwindcss').Config}*/
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "selector",
  theme: {
    extend: {
      gridTemplateColumns: {
        editor: "250px 1fr",
      },
      gridTemplateRows: {
        editor: "45px 45px 1fr 45px",
      },
    },
  },

  plugins: [],
}

module.exports = config
