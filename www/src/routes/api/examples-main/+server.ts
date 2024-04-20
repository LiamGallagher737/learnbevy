import { env } from "$env/dynamic/private";
import { Octokit } from "@octokit/core";
import type { Endpoints } from "@octokit/types";
import { error, type RequestHandler } from "@sveltejs/kit";

export type ExampleList = { value: string; label: string }[];
type Branch = Endpoints["GET /repos/{owner}/{repo}/branches/{branch}"]["response"]["data"];
type Tree = Endpoints["GET /repos/{owner}/{repo}/git/trees/{tree_sha}"]["response"]["data"];

const octokit = new Octokit({
    auth: env.PRIVATE_GITHUB_TOKEN,
});

export const GET: RequestHandler = async () => {
    const branch: Branch = await octokit
        .request("https://api.github.com/repos/bevyengine/bevy/branches/main")
        .then((res) => res.data);
    const tree_url = branch.commit.commit.tree.url;
    const full_tree: Tree = await octokit.request(tree_url).then((res) => res.data);
    const examples_tree_url = full_tree.tree.find((entry) => entry.path === "examples")?.url;
    if (!examples_tree_url) return error(500);
    const examples_tree: Tree = await octokit
        .request(examples_tree_url + "?recursive=1")
        .then((res) => res.data);

    const examples: ExampleList = [];
    examples_tree.tree.forEach((entry) => {
        if (!entry.path) return;
        if (!entry.path.endsWith(".rs")) return;
        examples.push(createEntry(entry.path));
    });

    return new Response(JSON.stringify(examples), {
        headers: {
            "content-type": "application/json",
            "cache-control": "public, max-age=14400",
        },
    });
};

function createEntry(filePath: string) {
    let label = filePath
        .split("/")
        .map((part) =>
            part
                .split("_")
                .map((str) => str.charAt(0).toUpperCase() + str.slice(1))
                .join(" ")
        )
        .join(" / ")
        .replace(".rs", "");
    return { value: filePath, label };
}
