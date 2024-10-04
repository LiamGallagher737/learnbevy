<script lang="ts">
    import Paintbrush from "lucide-svelte/icons/paintbrush";
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

    interface Props {
        version?: Version;
        channel?: Channel;
    }

    let { version = DEFAULT_VERSION, channel = DEFAULT_CHANNEL }: Props = $props();

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
</script>

<div class="flex flex-row gap-4">
    <BasicTooltip tooltip="Format">
        <Button variant="outline" size="icon" on:click={() => formatCode()}>
            <Paintbrush class="h-4 w-4" />
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
