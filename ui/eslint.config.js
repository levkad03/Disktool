import js from "@eslint/js";
import vue from "eslint-plugin-vue";
import tseslint from "typescript-eslint";
import prettier from "eslint-config-prettier";

export default tseslint.config(
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...vue.configs["flat/recommended"],
  prettier,
  {
    files: ["**/*.vue"],
    languageOptions: {
      parserOptions: {
        parser: tseslint.parser,
      },
    },
  },
  {
    rules: {
      "vue/multi-word-component-names": "off", // Optional: disable if you have single-word components like App.vue
    },
  },
  {
    ignores: ["node_modules", "dist"],
  },
);
