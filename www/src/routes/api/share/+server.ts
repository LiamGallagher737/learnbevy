import { error, type RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ request, platform }) => {
    const data = await request.json();
    if (!data.code || !data.version || !data.channel)
        error(422, 'Missing fields');
    if (typeof data.code !== "string" || typeof data.version !== "string" || typeof data.channel !== "string")
        error(422, 'Invalid field types');

    const json = JSON.stringify(data);
    const id = await hashString(json);
    await platform?.env?.SHARES.put(id, json);
    return new Response(String(id));
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

