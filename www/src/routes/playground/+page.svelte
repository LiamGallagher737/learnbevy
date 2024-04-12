<script lang="ts">
    import Editor from '$lib/components/Editor.svelte';
    import Actions from './Actions.svelte';
    import Console, { type ConsoleItem } from './Console.svelte';
    import { Button } from '$lib/components/ui/button';
    import { Card } from '$lib/components/ui/card';
    import * as Resizable from '$lib/components/ui/resizable';
    import { play as load } from '$lib/play';
    import { toast } from 'svelte-sonner';
    import Settings from './Settings.svelte';
    import type { Version } from '$lib/versions';
    import type { Channel } from '$lib/channels';

    const gameCanvasParentId = 'game-container';
    let gameCanvasParent: HTMLDivElement;

    let editor: Editor;
    let code: string;

    let gameCanvas: HTMLCanvasElement | null = null;
    let wasm: any | null = null;

    let consoleItems: ConsoleItem[];
    let settings: { version: Version, channel: Channel };

    async function play() {
        if (wasm) wasm.__exit();
        if (gameCanvas) gameCanvas.remove();
        consoleItems = [];
        const promise: Promise<void> = new Promise(async (resolve, reject) => {
            let result = await load({
                code,
                version: settings.version,
                channel: settings.channel,
                parentId: gameCanvasParentId,
            });
            if (result.kind === 'Failed') {
                if (result.stderr) consoleItems = [{ kind: 'Stdout', text: result.stderr }];
                reject(result.message);
            } else {
                if (result.kind === 'Success') gameCanvas = result.gameCanvas;
                wasm = result.wasm;
                consoleItems = [{ kind: 'Stdout', text: result.stderr }];
                resolve();
            }
        });
        toast.promise(promise, {
            loading: 'Loading...',
            success: 'Built successfully',
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
                <div class="flex flex-row gap-4">
                    <Actions />
                    <Settings bind:settings />
                </div>
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
            <Card class="flex-grow overflow-auto p-4 font-mono text-sm">
                <Console bind:consoleItems />
            </Card>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
