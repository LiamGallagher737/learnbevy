<script lang="ts">
    import Editor from "$lib/components/Editor.svelte";
    import Sidebar, { selectedTab } from "$lib/components/Sidebar.svelte";
    import Console from "$lib/components/Console.svelte";
    import { Button } from "$lib/components/ui/button";
    import { Card } from "$lib/components/ui/card";
    import * as Resizable from "$lib/components/ui/resizable";
    import { play as load } from "$lib/play";
    import { toast } from "svelte-sonner";
    import { consoleItems } from "$lib/components/console";
    import { editorCode } from "$lib/components/editor";
    import { onMount, tick } from "svelte";
    import { DEFAULT_VERSION } from "$lib/versions";
    import { DEFAULT_CHANNEL } from "$lib/channels";
    import Actions from "$lib/components/Actions.svelte";
    import { wasmBindings } from "$lib/play";

    const gameCanvasParentId = "game-container";
    let gameCanvasParent: HTMLDivElement = $state();

    let processingRequest = $state(false);

    let editor: Editor = $state();
    onMount(() => {
        selectedTab.subscribe(async (newValue) => {
            if (newValue !== "editor") return;
            await tick();
            editor.layout();
        });
    });

    let gameCanvas: HTMLCanvasElement | null = null;

    async function play() {
        if ($wasmBindings) $wasmBindings.exit();
        if (gameCanvas) gameCanvas.remove();
        consoleItems.set([]);
        processingRequest = true;
        const promise: Promise<void> = new Promise(async (resolve, reject) => {
            let result = await load({
                code: $editorCode,
                version: DEFAULT_VERSION,
                channel: DEFAULT_CHANNEL,
                parentId: gameCanvasParentId,
            });
            processingRequest = false;
            if (result.kind === "Failed") {
                if (result.stderr) consoleItems.set([{ kind: "Stdout", text: result.stderr }]);
                reject(result.message);
            } else {
                if (result.kind === "Success") gameCanvas = result.gameCanvas;
                consoleItems.set([{ kind: "Stdout", text: result.stderr }]);
                resolve();
            }
        });
        toast.promise(promise, {
            loading: "Loading...",
            success: "Built successfully",
            error: (err) => {
                return err as string;
            },
        });
    }

    function resizeGameCanvas() {
        if (!gameCanvas) return;
        gameCanvas.style.width = `${gameCanvasParent.clientWidth}px`;
        gameCanvas.style.height = `${gameCanvasParent.clientWidth * (9 / 16)}px`;
    }
</script>

<svelte:head>
    <title>Bevy Playground</title>
    <meta name="description" content="Experiment with Bevy apps in your browser" />
</svelte:head>

<div class="h-screen p-4">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane
            defaultSize={70}
            minSize={40}
            onResize={() => {
                if ($selectedTab === "editor") editor.layout();
                resizeGameCanvas();
            }}
            class="flex flex-col gap-4"
        >
            <Card class="flex flex-row justify-between p-4">
                <Button
                    class="font-semibold transition"
                    bind:disabled={processingRequest}
                    on:click={play}>Play</Button
                >
                <Actions />
            </Card>
            <div class="flex h-full w-full gap-4 overflow-hidden">
                <Card class="h-full w-12">
                    <Sidebar tabs={["editor"]} />
                </Card>
                <!-- The 4rem in calc() comes from 3rem sidebar + 1rem gap,
                flex-grow won't work because of the editor -->
                <Card class="h-full w-[calc(100%-4rem)] p-4">
                    {#if $selectedTab === "editor"}
                        <Editor bind:this={editor} />
                    {/if}
                </Card>
            </div>
        </Resizable.Pane>
        <Resizable.Handle withHandle class="mx-4" />
        <Resizable.Pane defaultSize={30} minSize={20} class="flex flex-col gap-4">
            <Card class="aspect-video">
                <div
                    bind:this={gameCanvasParent}
                    id={gameCanvasParentId}
                    class="h-full w-full"
                ></div>
            </Card>
            <Card class="flex-grow overflow-auto p-4 font-mono text-sm">
                <Console />
            </Card>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
