<script lang="ts">
    import type { Snippet } from "svelte";

    type Props = {
        label: Snippet;
        children: Snippet;
    };
    let { label, children } = $props();
</script>

<label>
    <span class="mr-4 font-bold text-lg">{@render label()}</span>
    {@render children()}
</label>
