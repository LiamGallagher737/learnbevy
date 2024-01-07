export const VERSIONS = ["0.12", "0.11", "0.10"] as const;
export type Version = typeof VERSIONS[number];
export const DEFAULT_VERSION = VERSIONS[0];
