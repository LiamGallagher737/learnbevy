<script lang="ts">
    import Editor from '$lib/components/Editor.svelte';
    import { Button } from '$lib/components/ui/button';
    import { Card } from '$lib/components/ui/card';
    import * as Resizable from '$lib/components/ui/resizable';
    import Actions from './Actions.svelte';

    let editor: Editor;
    let code: string;
</script>

<div class="h-screen p-4">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane
            defaultSize={60}
            minSize={20}
            onResize={() => editor.layout()}
            class="flex flex-col gap-4"
        >
            <Card class="flex flex-row justify-between p-4">
                <Button>Play</Button>
                <Actions />
            </Card>
            <Card class="h-full p-4">
                <Editor bind:this={editor} bind:code />
            </Card>
        </Resizable.Pane>
        <Resizable.Handle withHandle class="mx-4" />
        <Resizable.Pane defaultSize={40} minSize={20} class="flex flex-col gap-4">
            <Card class="aspect-video"></Card>
            <Card class="flex-grow overflow-auto p-4 text-sm"></Card>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
