import { defineConfig, presetAttributify, presetWind4 } from "unocss";

export default defineConfig({
    presets: [presetWind4(), presetAttributify()],
    preflights: [
        {
            getCSS: () => `
            html, body {
                height: 100vh;
            }

            body {
                background: #1a1a1a;
                color: #dadada;
            }`
        }
    ]
});
