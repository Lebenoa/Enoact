<script lang="ts">
    import "uno.css";
    import { activeTab, Tab } from "$lib/stores/active-tab.svelte";
    import Button from "$lib/components/Button.svelte";

    let { children } = $props();
</script>

<div class="flex w-full h-full flex-col items-center justify-center">
    <nav>
        <ol class="flex flex-row items-center justify-center gap-2">
            <li>
                <Button
                    active={activeTab.current === Tab.HOME}
                    onclick={() => {
                        activeTab.current = Tab.HOME;
                    }}
                >
                    New Activity
                </Button>
            </li>
            <li>
                <Button
                    active={activeTab.current === Tab.ACTIVITY}
                    onclick={() => {
                        activeTab.current = Tab.ACTIVITY;
                    }}
                >
                    Activities
                </Button>
            </li>
            <li>
                <Button
                    active={activeTab.current === Tab.SETTINGS}
                    onclick={() => {
                        activeTab.current = Tab.SETTINGS;
                    }}
                >
                    Settings
                </Button>
            </li>
        </ol>
    </nav>
    {@render children()}
</div>
