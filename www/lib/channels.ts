const CONST_CHANNELS = ["stable", "nightly"] as const;
export type Channel = typeof CONST_CHANNELS[number];
export const DEFAULT_CHANNEL: Channel = "nightly";
export const CHANNELS = CONST_CHANNELS as unknown as Channel[];
