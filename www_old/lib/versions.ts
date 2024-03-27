const CONST_VERSIONS = ["0.13", "0.12", "0.11", "0.10"] as const;
export type Version = typeof CONST_VERSIONS[number];
export const DEFAULT_VERSION = CONST_VERSIONS[0];
export const VERSIONS = CONST_VERSIONS as unknown as Version[];
