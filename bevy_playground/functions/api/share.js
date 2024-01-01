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
    const id = generateId(8);
    await context.env.BEVY_PLAYGROUND_SHARES.put(id, code);
    return new Response(JSON.stringify({ id }));
}

/**
 * @param {number} length 
 * @returns {string}
 */
function generateId(length) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    let counter = 0;
    while (counter < length) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
        counter += 1;
    }
    return result;
}
