"use server";

export async function formatCode(code: string) {
    const res = await fetch("https://rustfmt-api.fly.dev/format", {
        method: "POST",
        body: JSON.stringify({ code }),
        headers: {
            "Content-Type": "application/json",
        }
    });

    return await res.json() as FmtResponse;
}

type FmtResponse = FmtSuccess | FmtUserError | FmtServerError;

type FmtSuccess = {
    kind: "Success";
    formatted_code: string;
}

type FmtUserError = {
    kind: "UserError";
    stderr: string,
}

type FmtServerError = {
    kind: "ServerError";
}
