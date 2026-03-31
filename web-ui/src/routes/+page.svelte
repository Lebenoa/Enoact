<script lang="ts">
    import Home from "./Home.svelte";

    import { Tab, activeTab } from "$lib/stores/active-tab.svelte";
    import { fade } from "svelte/transition";
    import Activities from "./Activities.svelte";
    import Settings from "./Settings.svelte";
</script>

{#key activeTab.current}
    <div
        class="flex w-full h-full flex-row items-center justify-center gap-8 overflow-y-hidden"
        in:fade
    >
        {#if activeTab.current == Tab.HOME}
            <Home />
        {:else if activeTab.current == Tab.ACTIVITY}
            <Activities />
        {:else if activeTab.current == Tab.SETTINGS}
            <Settings />
        {/if}
    </div>
{/key}
