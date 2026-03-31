// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
    declare namespace svelteHTML {
        import type { AttributifyAttributes } from "@unocss/preset-attributify";

        type HTMLAttributes = AttributifyAttributes;
    }
}

export { };
