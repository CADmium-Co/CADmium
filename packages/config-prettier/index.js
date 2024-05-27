module.exports = {
  singleQuote: false,
  trailingComma: "all",
  bracketSpacing: false,
  arrowParens: "avoid",
  useTabs: false,
  tabWidth: 2,
  printWidth: 160,
  semi: false,
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
