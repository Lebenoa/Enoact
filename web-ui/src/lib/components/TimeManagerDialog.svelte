<script lang="ts">
    type Props = {
        ondialogclose: () => void;
        onconfirm: (value: any) => void;
    };
    let { ondialogclose, onconfirm }: Props = $props();

    import { fade, scale } from "svelte/transition";

    import Input from "./Input.svelte";
    import Button from "./Button.svelte";
    import Label from "./Label.svelte";
    import type { TimeManagerValue } from "$lib/types";

    let time: TimeManagerValue = $state({
        operation: "add",
        hours: 0,
        minutes: 0,
        seconds: 0
    });
</script>

<div class="absolute w-full h-full flex items-center justify-center">
    <button
        class="absolute w-full h-full text-opacity-0 bg-black opacity-70 z-1"
        transition:fade={{ duration: 300 }}
        onclick={ondialogclose}
        title="Close Dialog"
    >
    </button>
    <form
        class="flex flex-col gap-2 items-center justify-center z-2 px-6 py-4 bg-gray-900 shadow-lg"
        onsubmit={() => onconfirm(time)}
        transition:scale
    >
        <Button
            type="button"
            onclick={() => {
                if (time.operation === "add") {
                    time.operation = "subtract";
                } else if (time.operation === "subtract") {
                    time.operation = "add";
                }
            }}
        >
            {#if time.operation == "add"}
                + Add
            {:else if time.operation == "subtract"}
                - Subtract
            {/if}
            from now
        </Button>
        <div class="flex flex-row items-center justify-center gap-2">
            <Label>
                {#snippet label()}
                    Hour(s)
                {/snippet}
                <Input class="text-center" type="number" bind:value={time.hours} />
            </Label>
            <Label>
                {#snippet label()}
                    Minute(s)
                {/snippet}
                <Input class="text-center" type="number" bind:value={time.minutes} />
            </Label>
            <Label>
                {#snippet label()}
                    Seconds(s)
                {/snippet}
                <Input class="text-center" type="number" bind:value={time.seconds} />
            </Label>
        </div>
        <div class="flex flex-row gap-2">
            <Button type="button" varaint="danger" onclick={ondialogclose}>Cancel</Button>
            <Button type="submit" varaint="success">Confirm</Button>
        </div>
    </form>
</div>
