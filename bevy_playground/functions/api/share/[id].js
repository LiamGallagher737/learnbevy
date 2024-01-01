/**
 * @param {{ request: Request, env: *, params: { id: string } }} context
 * @returns {Response}
 */
export async function onRequest(context) {
    const id = context.params.id;
    const code = await context.env.BEVY_PLAYGROUND_SHARES.get(id);
    if (!code) {
        return new Response("Not Found", {
            status: 404,
        });
    }
    return new Response(JSON.stringify({ code }));
}