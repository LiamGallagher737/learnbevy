import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ params }) => {
    return new Response(
        JSON.stringify({
            crates: [
                { name: "bevy", version: params.bevyVersion },
                { name: "rand", version: "0.8.5" },
            ],
        })
    );
};
