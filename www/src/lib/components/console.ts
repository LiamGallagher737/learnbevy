import { writable } from "svelte/store";

export type ConsoleItem = Stdout | ConsoleLog;
type Stdout = {
    kind: "Stdout";
    text: string;
};
export type LogLevel = "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR";
type ConsoleLog = {
    kind: "Log";
    level: LogLevel;
    location: string;
    message: string;
};
export const consoleItems = writable<ConsoleItem[]>([]);
