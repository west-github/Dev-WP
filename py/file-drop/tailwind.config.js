import plugins from "tailwindcss/plugin"

/** @type {import('tailwindcss').Config} */
export const content = ["./templates/**/*.html"]
export const theme = {
  extend: {},
}
export const plugins = [plugins(function ({ addComponents }) {
  addComponents({
    ".center": {
      display: "flex",
      justifyContent: "center",
      alignItems: "center"
    }
  })
})]

