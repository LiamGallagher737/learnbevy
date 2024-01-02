/**
 * @param {{ request: Request, env: * }} context
 * @returns {Response}
 */
export async function onRequestPost(context) {
    const code = await context.request.text();
    if (!code) {
        return new Response("Must have a text body", {
            status: 400,
        });
    }
    const id = await hashString(code);
    await context.env.BEVY_PLAYGROUND_SHARES.put(id, code);
    return new Response(JSON.stringify({ id }));
}

/**
 * @param {string} message
 */
async function hashString(message) {
    const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
    const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
    const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
    const hashHex = hashArray
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");
    return hashHex;
}
