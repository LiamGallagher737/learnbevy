"use server";

import { Channel } from "@/lib/channels";
import { Version } from "@/lib/versions";
import { kv } from "@vercel/kv";

export async function createShare(code: string, version: Version, channel: Channel) {
    const id = await hashString(code + version + channel);
    try {
        await kv.set(id, { code, version, channel });
    } catch {
        throw new Error("Internal server error");
    }
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
