<script lang="ts">
    import Editor from "$lib/components/Editor.svelte";
    import Actions from "./Actions.svelte";
    import Sidebar from "./Sidebar.svelte";
    import Settings, { settings } from "./Settings.svelte";
    import Examples from "./Examples.svelte";
    import Console from "$lib/components/Console.svelte";
    import { Button } from "$lib/components/ui/button";
    import { Card } from "$lib/components/ui/card";
    import * as Resizable from "$lib/components/ui/resizable";
    import { play as load } from "$lib/play";
    import { toast } from "svelte-sonner";
    import { consoleItems } from "$lib/components/console";
    import { editorCode } from "$lib/components/editor";
    import type { PageData } from "./$types";
    import { onMount } from "svelte";

    export let data: PageData;
    if (data.code) editorCode.set(data.code);
    if (data.version && data.channel)
        settings.set({ version: data.version, channel: data.channel });
    onMount(() => {
        if (data.message) toast.error(data.message);
    });

    const gameCanvasParentId = "game-container";
    let gameCanvasParent: HTMLDivElement;

    let processingRequest = false;

    let editor: Editor;

    let gameCanvas: HTMLCanvasElement | null = null;
    let wasm: any | null = null;

    async function play() {
        if (wasm) wasm.__exit();
        if (gameCanvas) gameCanvas.remove();
        consoleItems.set([]);
        processingRequest = true;
        const promise: Promise<void> = new Promise(async (resolve, reject) => {
            let result = await load({
                code: $editorCode,
                version: $settings.version,
                channel: $settings.channel,
                parentId: gameCanvasParentId,
            });
            processingRequest = false;
            if (result.kind === "Failed") {
                if (result.stderr) consoleItems.set([{ kind: "Stdout", text: result.stderr }]);
                reject(result.message);
            } else {
                if (result.kind === "Success") gameCanvas = result.gameCanvas;
                wasm = result.wasm;
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
                editor.layout();
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
                <div class="flex flex-row gap-4">
                    <Examples />
                    <Actions />
                    <Settings />
                </div>
            </Card>
            <div class="flex h-full w-full gap-4">
                <Card class="h-full w-12">
                    <Sidebar />
                </Card>
                <!-- The 4rem in calc() comes from 3rem sidebar + 1rem gap -->
                <Card class="h-full w-[calc(100%-4rem)] p-4">
                    <Editor bind:this={editor} />
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
