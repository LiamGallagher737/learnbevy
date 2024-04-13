import { toast } from "svelte-sonner";
import { consoleItems } from "$lib/components/console";
import { get } from "svelte/store";
import { editorCode } from "./components/editor";

export async function formatCode() {
    const promise = new Promise(async (resolve, reject) => {
        const res = await fetch("https://rustfmt-api.fly.dev/format", {
            method: "POST",
            body: JSON.stringify({ code: get(editorCode) }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const result = (await res.json()) as FmtResponse;
        if (result.kind === "Success") {
            editorCode.set(result.formatted_code);
            resolve(result);
        } else {
            if (result.kind === "UserError")
                consoleItems.update((items) => [...items, { kind: "Stdout", text: result.stderr }]);
            reject(result);
        }
    });
    toast.promise(promise, {
        loading: "Loading...",
        success: "Formatted successfully",
        error: (e) => {
            const err = e as FmtError;
            if (err.kind === "UserError") return "Code could not be formatted";
            return "Something went wrong on our end";
        },
    });
}

type FmtResponse = FmtSuccess | FmtUserError | FmtServerError;
type FmtError = FmtUserError | FmtServerError;

type FmtSuccess = {
    kind: "Success";
    formatted_code: string;
};

type FmtUserError = {
    kind: "UserError";
    stderr: string;
};

type FmtServerError = {
    kind: "ServerError";
};
