/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../seedphrase_core/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/forms")],
};
