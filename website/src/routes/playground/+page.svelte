<script lang="ts">
    import * as Resizable from "@/components/ui/resizable";
    import { Card } from "@/components/ui/card";
</script>

<div class="h-screen p-4">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane defaultSize={70} minSize={40} class="flex flex-col gap-4">
            One
        </Resizable.Pane>

        <Resizable.Handle withHandle class="mx-4" />

        <Resizable.Pane defaultSize={30} minSize={20} class="flex flex-col gap-4">
            <Card class="aspect-video"></Card>
            <Card class="flex-grow overflow-auto p-4 font-mono text-sm"></Card>
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
