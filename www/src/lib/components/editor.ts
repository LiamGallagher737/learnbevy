import { writable } from "svelte/store";
import { DEFAULT_CODE } from "$lib/default-code";

export const editorCode = writable(DEFAULT_CODE);
