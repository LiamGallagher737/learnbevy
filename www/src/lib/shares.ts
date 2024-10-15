import { openKv } from "@deno/kv";

export async function create(code: string): Promise<string> {
    const msgUint8 = new TextEncoder().encode(code);
    const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashHex = hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");

    const kv = await openKv();
    await kv.set(["shares", hashHex], code);

    return hashHex;
}

export async function get(key: string): Promise<string | null> {
    const kv = await openKv();
    const result = await kv.get(["shares", key]);
    return result.value as string ?? null;
}
