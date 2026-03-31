<script lang="ts">
    import Button from "$lib/components/Button.svelte";
    import Input from "$lib/components/Input.svelte";
    import Label from "$lib/components/Label.svelte";
    import Select from "$lib/components/Select.svelte";
    import TimeManagerDialog from "$lib/components/TimeManagerDialog.svelte";

    import { type Activity, ActivityType, type Assets, StatusDisplayType } from "$lib/types";

    import { LocalStorageStore } from "$lib/stores/local-storage.svelte";

    import {
        formatActivityType,
        formatSecondsToHumanReadable,
        unixMillisToSeconds
    } from "$lib/utils";
    import { onMount } from "svelte";
    import { scale } from "svelte/transition";

    let activity: Activity = $state({});
    let accordionState = $state({
        timestamps: false,
        assets: false
    });
    let timeDialogOpen = $state(false);
    let now = $state(0);

    let operateOn: "start" | "end" | null = null;

    function onAssetsInput(key: keyof Assets, value: string) {
        if (!activity.assets) {
            activity.assets = {};
        }
        activity.assets[key] = value;
    }

    async function onPresenceSubmit(e: SubmitEvent) {
        e.preventDefault();

        try {
            const response = await fetch("/api/set-presence", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    activity
                })
            });

            if (!response.ok) {
                const errorText = await response.text();
                console.error("Failed to set presence:", errorText);
                alert("Error: " + errorText);
            } else {
                const result = await response.text();
                console.log("Presence set successfully:", result);
                alert("Presence updated successfully!");
            }
        } catch (error) {
            console.error("Error submitting presence:", error);
            alert("An unexpected error occurred.");
        }
    }

    onMount(() => {
        now = Date.now();
        setInterval(() => {
            now = Date.now();
        }, 1000);
    });
</script>

{#if timeDialogOpen}
    <TimeManagerDialog
        ondialogclose={() => (timeDialogOpen = false)}
        onconfirm={(value) => {
            timeDialogOpen = false;
            if (operateOn === null) return;
            if (!activity.timestamps) {
                activity.timestamps = {};
            }

            let convertedValue: number;
            if (value.operation === "add") {
                convertedValue = (value.hours * 3600 + value.minutes * 60 + value.seconds) * 1000;
            } else if (value.operation === "subtract") {
                convertedValue = -(value.hours * 3600 + value.minutes * 60 + value.seconds) * 1000;
            } else {
                return;
            }

            if (operateOn === "start") {
                if (!activity.timestamps.start) {
                    activity.timestamps.start = Date.now();
                }
                activity.timestamps.start += convertedValue;
            } else if (operateOn === "end") {
                if (!activity.timestamps.end) {
                    activity.timestamps.end = Date.now();
                }
                activity.timestamps.end += convertedValue;
            }
        }}
    />
{/if}

<form
    class="w-full flex flex-col gap-4 h-full overflow-y-auto"
    lg="w-1/2"
    onsubmit={onPresenceSubmit}
>
    <div class="w-full flex flex-col gap-2">
        <Label>
            {#snippet label()}
                <span>App Name</span>
            {/snippet}
            <Input type="text" bind:value={activity.name} />
        </Label>
        <Label>
            {#snippet label()}
                <span>State</span>
            {/snippet}
            <Input type="text" bind:value={activity.state} />
        </Label>
        {#if activity.state}
            <div class="w-full" transition:scale>
                <Label>
                    {#snippet label()}
                        <span>State URL</span>
                    {/snippet}
                    <Input type="text" bind:value={activity.state_url} />
                </Label>
            </div>
        {/if}
        <Label>
            {#snippet label()}
                <span>Details</span>
            {/snippet}
            <Input type="text" bind:value={activity.details} />
        </Label>
        {#if activity.details}
            <div class="w-full" transition:scale>
                <Label>
                    {#snippet label()}
                        <span>Details URL</span>
                    {/snippet}
                    <Input type="text" bind:value={activity.details_url} />
                </Label>
            </div>
        {/if}
        <Label>
            {#snippet label()}
                <span>Activity Type</span>
            {/snippet}
            <Select
                onchange={(e) => {
                    const enumValue = parseInt(e.currentTarget.value);
                    if (isNaN(enumValue)) {
                        activity.activity_type = ActivityType.Playing;
                    } else {
                        activity.activity_type = enumValue as ActivityType;
                    }
                }}
            >
                <option value="0">Playing</option>
                <option value="2">Listening</option>
                <option value="3">Watching</option>
                <option value="5">Competing</option>
            </Select>
        </Label>
        <Label>
            {#snippet label()}
                <span>Status Display Type</span>
            {/snippet}
            <Select
                onchange={(e) => {
                    const enumValue = parseInt(e.currentTarget.value);
                    if (isNaN(enumValue)) {
                        activity.status_display_type = StatusDisplayType.Name;
                    } else {
                        activity.status_display_type = enumValue as StatusDisplayType;
                    }
                }}
            >
                <option value="0">Name</option>
                <option value="1">State</option>
                <option value="2">Details</option>
            </Select>
        </Label>
        <Button
            type="button"
            additionalClass="w-full"
            active={accordionState.timestamps}
            onclick={() => {
                accordionState.timestamps = !accordionState.timestamps;
                if (!activity.timestamps) {
                    activity.timestamps = {};
                }
            }}
        >
            <h3 class="font-bold text-2xl">Timestamps</h3>
        </Button>
        {#if accordionState.timestamps}
            <div class="w-full" transition:scale>
                <Label>
                    {#snippet label()}
                        <span>Start</span>
                    {/snippet}
                    <div class="flex flex-row w-full">
                        <Input type="text" bind:value={activity.timestamps!.start} />
                        <Button
                            type="button"
                            onclick={() => {
                                if (!activity.timestamps) activity.timestamps = {};
                                activity.timestamps.start = Date.now();
                                operateOn = "start";
                                timeDialogOpen = true;
                            }}
                        >
                            Now
                        </Button>
                    </div>
                </Label>
                <Label>
                    {#snippet label()}
                        <span>End</span>
                    {/snippet}
                    <div class="flex flex-row w-full">
                        <Input type="text" bind:value={activity.timestamps!.end} />
                        <Button
                            type="button"
                            onclick={() => {
                                if (!activity.timestamps) activity.timestamps = {};
                                activity.timestamps.end = Date.now();
                                operateOn = "end";
                                timeDialogOpen = true;
                            }}
                        >
                            Now
                        </Button>
                    </div>
                </Label>
            </div>
        {/if}
        <Button
            type="button"
            additionalClass="w-full"
            active={accordionState.assets}
            onclick={() => (accordionState.assets = !accordionState.assets)}
        >
            <h3 class="font-bold text-2xl">Assets</h3>
        </Button>
        {#if accordionState.assets}
            <div class="w-full" transition:scale>
                <Label>
                    {#snippet label()}
                        <span>Large Image Key</span>
                    {/snippet}
                    <Input
                        type="text"
                        value={activity.assets?.large_image ? activity.assets.large_image : ""}
                        oninput={(e) => {
                            onAssetsInput("large_image", e.currentTarget.value);
                        }}
                    />
                </Label>
                {#if activity.assets?.large_image}
                    <div transition:scale>
                        <Label>
                            {#snippet label()}
                                <span>Large Image Text</span>
                            {/snippet}
                            <Input
                                type="text"
                                value={activity.assets.large_text}
                                oninput={(e) => {
                                    onAssetsInput("large_text", e.currentTarget.value);
                                }}
                            />
                        </Label>
                        <Label>
                            {#snippet label()}
                                <span>Large Image URL</span>
                            {/snippet}
                            <Input
                                type="text"
                                value={activity.assets.large_url}
                                oninput={(e) => {
                                    onAssetsInput("large_url", e.currentTarget.value);
                                }}
                            />
                        </Label>
                    </div>
                {/if}
                <Label>
                    {#snippet label()}
                        <span>Small Image Key</span>
                    {/snippet}
                    <Input
                        type="text"
                        value={activity.assets?.small_image ? activity.assets.small_image : ""}
                        oninput={(e) => {
                            onAssetsInput("small_image", e.currentTarget.value);
                        }}
                    />
                </Label>
                {#if activity.assets?.small_image}
                    <div transition:scale>
                        <Label>
                            {#snippet label()}
                                <span>Small Image Text</span>
                            {/snippet}
                            <Input
                                type="text"
                                value={activity.assets.small_text}
                                oninput={(e) => {
                                    onAssetsInput("small_text", e.currentTarget.value);
                                }}
                            />
                        </Label>
                        <Label>
                            {#snippet label()}
                                <span>Small Image URL</span>
                            {/snippet}
                            <Input
                                type="text"
                                value={activity.assets.small_url}
                                oninput={(e) => {
                                    onAssetsInput("small_url", e.currentTarget.value);
                                }}
                            />
                        </Label>
                    </div>
                {/if}
            </div>
        {/if}
    </div>
    <Button variant="success" additionalClass="grid-col-span-2 mt-4" type="submit">Update</Button>
</form>
<div class="flex flex-col h-fit my-auto gap-2 bg-black/20 px-4 py-2 cursor-pointer">
    <p>
        {formatActivityType(
            activity.activity_type ? activity.activity_type : ActivityType.Playing,
            activity.status_display_type ? activity.status_display_type : StatusDisplayType.Name,
            activity.name ? activity.name : "[Default App Name]",
            activity.state,
            activity.details
        )}
    </p>
    <div class="flex flex-row">
        {#if activity.assets?.large_image}
            <div class="relative group flex flex-col items-center w-24 h-24">
                {#if activity.assets.large_text}
                    <p class="bg-black px-2 py-1 text-white hidden mt--8 group-hover:block">
                        {activity.assets.large_text}
                    </p>
                {/if}
                <img
                    class="absolute w-24 h-24 rounded-lg"
                    src={activity.assets.large_image}
                    alt={activity.assets.large_text ? activity.assets.large_text : ""}
                />
                {#if activity.assets?.small_image}
                    <div class="relative group flex flex-col mt-auto ml-auto items-center">
                        {#if activity.assets.small_text}
                            <p
                                class="absolute bg-black px-2 py-1 text-white hidden mt--8 group-hover:block"
                            >
                                {activity.assets.small_text}
                            </p>
                        {/if}
                        <img
                            class="w-6 h-6 rounded-4xl"
                            src={activity.assets.small_image}
                            alt={activity.assets.small_text ? activity.assets.small_text : ""}
                        />
                    </div>
                {/if}
            </div>
        {/if}
        <div class="flex flex-col ml-2">
            <h4 class="text-xl font-bold">{activity.state}</h4>
            <p>{activity.details}</p>
            {#if activity.timestamps?.start}
                <div class="flex flex-row gap-2">
                    <p>
                        {formatSecondsToHumanReadable(
                            unixMillisToSeconds(now - activity.timestamps.start)
                        )}
                    </p>
                    {#if activity.timestamps.end}
                        <progress
                            value={unixMillisToSeconds(now - activity.timestamps.start)}
                            max={unixMillisToSeconds(
                                activity.timestamps.end - activity.timestamps.start
                            )}
                        ></progress>
                        <p>
                            {formatSecondsToHumanReadable(
                                unixMillisToSeconds(
                                    activity.timestamps.end - activity.timestamps.start
                                )
                            )}
                        </p>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</div>
