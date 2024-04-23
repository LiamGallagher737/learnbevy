export interface Env {}

export default {
    async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
        if (request.method === "OPTIONS" || request.method === "GET") {
            return await fetch(request);
        }
        if (request.method !== "POST") {
            return new Response("Method not allowed", { status: 405 });
        }

        async function sha256(text: string) {
            const msgBuffer = await new TextEncoder().encode(text);
            const hashBuffer = await crypto.subtle.digest("SHA-256", msgBuffer);
            return [...new Uint8Array(hashBuffer)]
                .map((b) => b.toString(16).padStart(2, "0"))
                .join("");
        }

        const body = await request.clone().text();
        const hash = await sha256(body);
        const cacheUrl = new URL(request.url);
        cacheUrl.pathname = "/compiles" + cacheUrl.pathname + hash;
        const cacheKey = new Request(cacheUrl.toString(), {
            headers: request.headers,
            method: "GET",
        });

        const cache = caches.default;
        let response = await cache.match(cacheKey);

        if (!response) {
            response = await fetch(request);
            ctx.waitUntil(cache.put(cacheKey, response.clone()));
        }
        return response;
    },
};
