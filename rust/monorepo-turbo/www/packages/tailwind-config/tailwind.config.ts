import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";
import { display_styles } from "./styles";

const plugin_definition = plugin(function ({ addComponents }) {
    addComponents({ ...display_styles });
});

const config: Omit<Config, "content"> = {
    plugins: [plugin_definition],
};

export default config;
