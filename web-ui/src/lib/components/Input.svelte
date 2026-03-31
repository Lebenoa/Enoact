<script lang="ts">
    import type { HTMLInputAttributes } from "svelte/elements";

    interface Props extends HTMLInputAttributes {
        additionalClass?: string;
    }
    let { value = $bindable(), ...rest }: Props = $props();
</script>

<input
    p="x-4 y-2"
    border="~ cyan-500"
    w="full"
    {...rest}
    un-disabled="text-gray-500 border-gray-500 cursor-not-allowed"
    bind:value
/>
