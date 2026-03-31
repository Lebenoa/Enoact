export enum Tab {
    HOME = "home",
    SETTINGS = "settings",
    ACTIVITY = "activity",
}

type ActiveTab = {
    current: Tab;
};

export const activeTab: ActiveTab = $state({
    current: Tab.HOME
})
