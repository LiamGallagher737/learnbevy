"use server";

import { Version } from "@/lib/versions";

export async function createShare(code: string, version: Version) {
    const id = await hashString(code + version);
    await process.env.SHARES.put(id, code);
    return { id };
}

async function hashString(message: string) {
    const msgUint8 = new TextEncoder().encode(message);
    const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");
    return hashHex;
}
