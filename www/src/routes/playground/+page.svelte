<script lang="ts">
    import AssetExplorer from "./AssetExplorer.svelte";
    import CrateList from "./CrateList.svelte";
    import Editor from "$lib/components/Editor.svelte";
    import Actions from "$lib/components/Actions.svelte";
    import Sidebar, { selectedTab } from "$lib/components/Sidebar.svelte";
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
    import { onMount, tick } from "svelte";
    import About from "./About.svelte";

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
    onMount(() => {
        selectedTab.subscribe(async (newValue) => {
            if (newValue !== "editor") return;
            await tick();
            editor.layout();
        });
    });

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
                <div class="flex flex-row gap-4">
                    <Examples />
                    <Actions version={$settings.version} channel={$settings.channel} />
                    <Settings />
                </div>
            </Card>
            <div class="flex h-full w-full gap-4 overflow-hidden">
                <Card class="h-full w-12">
                    <Sidebar tabs={["editor", "assets", "crates", "about"]} />
                </Card>
                <!-- The 4rem in calc() comes from 3rem sidebar + 1rem gap,
                flex-grow won't work because of the editor -->
                <Card class="h-full w-[calc(100%-4rem)] p-4">
                    {#if $selectedTab === "editor"}
                        <Editor bind:this={editor} />
                    {:else if $selectedTab === "assets"}
                        <AssetExplorer />
                    {:else if $selectedTab === "crates"}
                        <CrateList />
                    {:else if $selectedTab === "about"}
                        <About />
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
