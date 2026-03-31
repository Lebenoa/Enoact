<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLButtonAttributes } from "svelte/elements";

    import clsx from "clsx";

    interface Props extends HTMLButtonAttributes {
        variant?: "primary" | "success" | "danger";
        additionalClass?: string;
        active?: boolean;
        additionalActiveClass?: string;
        additionalUnactiveClass?: string;
        children: Snippet;
        [key: string]: any;
    }

    const variantColors = {
        primary: {
            border: "border-cyan-500",
            notActive: {
                text: "text-cyan-500/80",
                hover: "lg:hover:bg-cyan-500/30 lg:hover:text-cyan-500"
            },
            active: {
                bg: "bg-cyan-500/50"
            }
        },
        success: {
            border: "border-green-500",
            notActive: {
                text: "text-green-500/80",
                hover: "lg:hover:bg-green-500/30 lg:hover:text-green-500"
            },
            active: {
                bg: "bg-green-500/50"
            }
        },
        danger: {
            border: "border-red-500",
            notActive: {
                text: "text-red-500/80",
                hover: "lg:hover:bg-red-500/30 lg:hover:text-red-500"
            },
            active: {
                bg: "bg-red-500/50"
            }
        }
    };

    let {
        additionalClass,
        variant = "primary",
        children,
        active = false,
        additionalActiveClass,
        additionalUnactiveClass,
        ...rest
    }: Props = $props();

    // svelte-ignore state_referenced_locally
    let selectedVariant = variantColors[variant];
</script>

<button
    class={clsx(
        "transition-colors duration-300 cursor-pointer border",
        selectedVariant.border,
        additionalClass,
        !active && selectedVariant.notActive.text,
        !active && selectedVariant.notActive.hover,
        !active && additionalUnactiveClass,
        active && selectedVariant.active.bg,
        active && additionalActiveClass
    )}
    p="x-4 y-2"
    un-disabled="border-gray-500 text-gray-500 cursor-not-allowed"
    {...rest}
>
    {@render children()}
</button>
