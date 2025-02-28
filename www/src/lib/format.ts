import { toast } from "svelte-sonner";
import { consoleItems } from "$lib/components/console";
import { get } from "svelte/store";
import { editorCode } from "./components/editor";
import { env } from "$env/dynamic/public";

export async function formatCode() {
    const promise = new Promise(async (resolve, reject) => {
        const host = env.PUBLIC_COMPILE_HOST ?? "https://slc.compute.learnbevy.com";
        const url = `${host}/format`;
        const res = await fetch(url, {
            method: "POST",
            body: JSON.stringify({ code: get(editorCode) }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const result = await res.json();
        if (res.status === 200) {
            const success = result as FmtSuccess;
            editorCode.set(success.formatted_code);
            resolve(success);
        } else {
            const err = result as FmtError;
            if (err.kind === "BadCode")
                consoleItems.update((items) => [...items, { kind: "Stdout", text: result.stderr }]);
            reject(result);
        }
    });
    toast.promise(promise, {
        loading: "Loading...",
        success: "Formatted successfully",
        error: (e) => {
            const err = e as FmtError;
            if (err.kind === "BadCode") return "Code could not be formatted";
            return "Something went wrong on our end";
        },
    });
}

type FmtError = FmtUserError | FmtServerError;

type FmtSuccess = {
    formatted_code: string;
};

type FmtUserError = {
    kind: "BadCode";
    stderr: string;
};

type FmtServerError = {
    kind: "Internal";
};
