const CONST_VERSIONS = ["main", "0.14"] as const;
export type Version = (typeof CONST_VERSIONS)[number];
export const DEFAULT_VERSION: Version = "0.14";
export const VERSIONS = CONST_VERSIONS as unknown as Version[];
