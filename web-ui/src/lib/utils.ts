import { ActivityType, StatusDisplayType } from "./types";

export function formatActivityType(activity_type: ActivityType, display_type: StatusDisplayType, app_name: string, state?: string, details?: string): string {
    let suffix: string;
    switch (display_type) {
        case StatusDisplayType.State:
            suffix = state ?? "";
            break;
        case StatusDisplayType.Details:
            suffix = details ?? "";
            break;
        default:
            suffix = app_name;
            break;
    }

    if (activity_type == ActivityType.Listening) {
        return `Listening to ${suffix}`;
    }

    return `${ActivityType[activity_type]} ${suffix}`;
}

export function unixMillisToSeconds(millis: number): number {
    return (millis / 1000)
}

export function formatSecondsToHumanReadable(seconds: number): string {
    if (seconds < 60) {
        return `0:${seconds < 10 ? "0" : ""}${Math.floor(seconds)}`;
    }

    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    if (minutes < 60) {
        return `${minutes}:${remainingSeconds < 10 ? "0" : ""}${remainingSeconds}`;
    }

    const hours = Math.floor(minutes / 60);
    const remainingMinutes = Math.floor(minutes % 60);
    return `${hours}:${remainingMinutes < 10 ? "0" : ""}${remainingMinutes}:${remainingSeconds < 10 ? "0" : ""}${remainingSeconds}`;
}
