/** @type {import("eslint").Linter.Config} */
module.exports = {
  root: true,
  extends: ["@repo/config-eslint/index.js", "plugin:svelte/recommended"],
  parserOptions: {
    extraFileExtensions: [".svelte"],
  },
  env: {
    browser: true,
    es2017: true,
    node: true,
  },
}
