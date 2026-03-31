<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLSelectAttributes } from "svelte/elements";

    interface Props extends HTMLSelectAttributes {
        children: Snippet;
    }

    let { value = $bindable(), children, ...rest }: Props = $props();
</script>

<select
    class="border border-cyan-500 px-4 py-2 cursor-pointer w-full"
    un-disabled="text-gray-500 border-gray-500 cursor-not-allowed"
    {...rest}
    bind:value
>
    {@render children()}
</select>
