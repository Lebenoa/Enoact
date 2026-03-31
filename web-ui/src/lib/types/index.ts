export type Timestamps = {
    start?: number;
    end?: number;
}

export type Party = {
    id?: string;
    size?: number[];
}

export type Assets = {
    large_image?: string;
    large_text?: string;
    large_url?: string;
    small_image?: string;
    small_text?: string;
    small_url?: string;
}

export type Secrets = {
    join?: string;
    spectate?: string;
    match?: string;
}

export type Button = {
    label?: string;
    url?: string;
}

export enum ActivityType {
    Playing = 0,
    Listening = 2,
    Watching = 3,
    Competing = 5,
}

export enum StatusDisplayType {
    Name = 0,
    State = 1,
    Details = 2,
}

export type Activity = {
    name?: string;
    state?: string;
    state_url?: string;
    details?: string;
    details_url?: string;
    timestamps?: Timestamps;
    party?: Party;
    assets?: Assets;
    secrets?: Secrets;
    buttons?: Button[];
    activity_type?: ActivityType;
    status_display_type?: StatusDisplayType;
}

export type ActiveAppIdsResponse = [string, Activity][];

export type TimeManagerValue = {
    operation: "add" | "subtract";
    hours: number;
    minutes: number;
    seconds: number;
}
