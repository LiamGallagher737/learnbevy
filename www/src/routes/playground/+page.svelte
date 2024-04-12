<script lang="ts">
    import Editor from '$lib/components/Editor.svelte';
    import { Button } from '$lib/components/ui/button';
    import { Card } from '$lib/components/ui/card';
    import * as Resizable from '$lib/components/ui/resizable';
    import Actions from './Actions.svelte';
    import { play as load } from '$lib/play';

    const gameCanvasParentId = 'game-container';
    let gameCanvasParent: HTMLDivElement;

    let editor: Editor;
    let code: string;

    let gameCanvas: HTMLCanvasElement | null = null;
    let wasm: any | null = null;

    async function play() {
        if (wasm) wasm.__exit();
        if (gameCanvas) gameCanvas.remove();
        let result = await load({
            code,
            version: '0.13',
            channel: 'nightly',
            parentId: gameCanvasParentId,
        });
        gameCanvas = result.gameCanvas;
        wasm = result.wasm;
    }

    function resizeGameCanvas() {
        if (!gameCanvas) return;
        gameCanvas.style.width = `${gameCanvasParent.clientWidth}px`;
        gameCanvas.style.height = `${gameCanvasParent.clientWidth * (9 / 16)}px`;
    }
</script>

<div class="h-screen p-4">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane
            defaultSize={60}
            minSize={20}
            onResize={() => {
                editor.layout();
                resizeGameCanvas();
            }}
            class="flex flex-col gap-4"
        >
            <Card class="flex flex-row justify-between p-4">
                <Button class="font-semibold" on:click={play}>Play</Button>
                <Actions />
            </Card>
            <Card class="h-full p-4">
                <Editor bind:this={editor} bind:code />
            </Card>
        </Resizable.Pane>
        <Resizable.Handle withHandle class="mx-4" />
        <Resizable.Pane defaultSize={40} minSize={20} class="flex flex-col gap-4">
            <Card class="aspect-video">
                <div
                    bind:this={gameCanvasParent}
                    id={gameCanvasParentId}
                    class="h-full w-full"
                ></div>
            </Card>
            <Card class="flex-grow overflow-auto p-4 text-sm"></Card>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
