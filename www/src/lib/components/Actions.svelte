<script lang="ts">
    import Paintbrush from "lucide-svelte/icons/paintbrush";
    import AlertBadge from "lucide-svelte/icons/badge-alert";
    import SpellCheck from "lucide-svelte/icons/spell-check-2";
    import Wrench from "lucide-svelte/icons/wrench";
    import Copy from "lucide-svelte/icons/copy";
    import Share from "lucide-svelte/icons/share";
    import BasicTooltip from "$lib/components/BasicTooltip.svelte";
    import { Button } from "$lib/components/ui/button";
    import { formatCode } from "$lib/format";
    import { editorCode } from "$lib/components/editor";
    import { toast } from "svelte-sonner";
    import { goto } from "$app/navigation";
    import { DEFAULT_VERSION, type Version } from "$lib/versions";
    import { DEFAULT_CHANNEL, type Channel } from "$lib/channels";
    import { consoleItems } from "$lib/components/console";
    import { env } from "$env/dynamic/public";

    export let version: Version = DEFAULT_VERSION;
    export let channel: Channel = DEFAULT_CHANNEL;

    async function copyCodeToClipboard() {
        await navigator.clipboard.writeText($editorCode);
        toast.success("Code copied to clipboard");
    }

    async function createCodeShare() {
        const promise: Promise<void> = new Promise(async (resolve, reject) => {
            let result = await fetch("/api/share", {
                method: "POST",
                body: JSON.stringify({
                    code: $editorCode,
                    version,
                    channel,
                }),
            });
            if (!result.ok) {
                reject();
            }
            const id = await result.text();
            await navigator.clipboard.writeText(`https://learnbevy.com/playground?share=${id}`);
            await goto(`?share=${id}`);
            resolve();
        });
        toast.promise(promise, {
            loading: "Loading...",
            success: "Copied url to clipboard",
            error: "Failed to create share",
        });
    }

    async function clippy(fix: boolean) {
        const host = env.PUBLIC_COMPILE_HOST ?? "https://slc.compute.learnbevy.com";
        const url = `${host}/clippy/${version}/${channel}`;

        const res = await fetch(url, {
            method: "POST",
            body: JSON.stringify({ code: $editorCode, fix }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const json = await res.json();

        if (res.status === 200) {
            if (fix) {
                editorCode.set(json.fixed_code);
            }
            consoleItems.update((items) => [...items, { kind: "Stdout", text: json.stderr }]);
        }
    }

    async function lint() {
        const host = env.PUBLIC_COMPILE_HOST ?? "https://slc.compute.learnbevy.com";
        const url = `${host}/lint/${version}/${channel}`;

        const res = await fetch(url, {
            method: "POST",
            body: JSON.stringify({ code: $editorCode }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const json = await res.json();

        if (res.status === 200) {
            consoleItems.update((items) => [...items, { kind: "Stdout", text: json.stderr }]);
        }
    }
</script>

<div class="flex flex-row gap-4">
    <BasicTooltip tooltip="Format">
        <Button variant="outline" size="icon" on:click={() => formatCode()}>
            <Paintbrush class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Bevy Lint">
        <Button variant="outline" size="icon" on:click={() => lint()}>
            <AlertBadge class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Clippy">
        <Button variant="outline" size="icon" on:click={() => clippy(false)}>
            <SpellCheck class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Clippy & Fix">
        <Button variant="outline" size="icon" on:click={() => clippy(true)}>
            <Wrench class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Copy">
        <Button variant="outline" size="icon" on:click={copyCodeToClipboard}>
            <Copy class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Share">
        <Button variant="outline" size="icon" on:click={createCodeShare}>
            <Share class="h-4 w-4" />
        </Button>
    </BasicTooltip>
</div>
