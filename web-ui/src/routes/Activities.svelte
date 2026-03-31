<script lang="ts">
    import type { ActiveAppIdsResponse } from "$lib/types";

    import { onMount } from "svelte";

    import Button from "$lib/components/Button.svelte";
    import Label from "$lib/components/Label.svelte";
    import Input from "$lib/components/Input.svelte";
    import { fade } from "svelte/transition";

    let activeTabId: string | undefined = $state(undefined);
    let activeAppIds: ActiveAppIdsResponse = $state([]);

    async function fetchActiveAppIds() {
        const response = await fetch("/api/active-app-ids");
        const data: ActiveAppIdsResponse = await response.json();
        activeAppIds = data;
        activeTabId = data.length > 0 ? data[0][0] : undefined;
    }

    async function clearActivity() {
        if (!activeTabId) return;
        const response = await fetch(`/api/clear-presence`, { method: "POST", body: activeTabId });
        if (response.ok) {
            activeAppIds = activeAppIds.filter(([id]) => id !== activeTabId);
            if (activeTabId === activeAppIds[0][0]) {
                activeTabId = activeAppIds[0][0];
            } else {
                activeTabId = undefined;
            }
        }
    }

    onMount(() => {
        fetchActiveAppIds();
    });
</script>

{#if activeAppIds.length == 0}
    <h2 class="text-4xl font-bold">No active HTTP activity!</h2>
{:else}
    <div class="flex flex-col items-center h-full w-full">
        {#each activeAppIds as [appId, activity], index (appId)}
            <div class="mb-4 flex flex-col justify-center w-full" lg-w="2/3">
                <Button
                    additionalClass="justify-self-center"
                    active={activeTabId == appId}
                    onclick={() => (activeTabId = appId)}
                >
                    {appId}
                </Button>
                {#if activeTabId == appId}
                    <div in:fade>
                        <Label>
                            {#snippet label()}
                                Name
                            {/snippet}
                            <Input type="text" value={activity.name} disabled />
                        </Label>
                        <Label>
                            {#snippet label()}
                                State
                            {/snippet}
                            <Input type="text" value={activity.state} disabled />
                        </Label>
                        <Label>
                            {#snippet label()}
                                Details
                            {/snippet}
                            <Input type="text" value={activity.details} disabled />
                        </Label>
                    </div>
                {/if}
            </div>
        {/each}
        {#if activeTabId}
            <div in:fade>
                <Button variant="danger" onclick={clearActivity}>Clear</Button>
            </div>
        {/if}
    </div>
{/if}
