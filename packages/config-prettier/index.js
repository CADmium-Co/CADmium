module.exports = {
  semi: false,
  singleQuote: false,
  useTabs: false,
  tabWidth: 2,
  trailingComma: "all",
  bracketSpacing: false,
  arrowParens: "avoid",
  printWidth: 160,
  plugins: ["prettier-plugin-svelte"],
  overrides: [
    {
      files: "*.svelte",
      options: {
        parser: "svelte",
      },
    },
  ],
}
