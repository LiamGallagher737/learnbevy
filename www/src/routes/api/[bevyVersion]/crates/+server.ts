import { error, type RequestHandler } from "@sveltejs/kit";
import TOML from 'smol-toml';

const EXCLUDE_CRATES = ["wasm-bindgen"];

export const GET: RequestHandler = async ({ params }) => {
    const url = `https://raw.githubusercontent.com/LiamGallagher737/learnbevy/main/images/manifests/${params.bevyVersion}.Cargo.toml`;
    const response = await fetch(url);

    if (response.status === 404) {
        error(404, {
            message: 'Not found',
        });
    }

    const text = await response.text();
    const manifest = TOML.parse(text);

    let crates = [];

    for (const [name, value] of Object.entries(manifest.dependencies)) {
        if (EXCLUDE_CRATES.includes(name)) continue;

        if (typeof value === "string") {
            crates.push({ name, version: value });
        } else if (typeof value === "object" && typeof value.version === "string") {
            crates.push({ name, version: value.version });
        } else {
            error(500, {
                message: "Failed to parse crate " + name,
            });
        }
    }

    return new Response(
        JSON.stringify({ crates })
    );
};
