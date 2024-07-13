import type { RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ params }) => {
    return new Response(
        JSON.stringify({
            crates: [
                { name: "bevy", versions: params.bevyVersion },
                { name: "rand", versions: "0.8.5" },
            ],
        })
    );
};
