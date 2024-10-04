<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Select from "$lib/components/ui/select";
    import * as Table from "$lib/components/ui/table";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import { topLevelFolders, assetMap } from "$lib/asset-explorer/asset-map";
    import { toast } from "svelte-sonner";

    type Folder = keyof typeof assetMap;
    let selectedFolder: { label: string; value: Folder } = $state({
        label: "textures",
        value: "textures",
    });

    function getFileName(path: string) {
        let name = path.split("/").pop()!.split(".")[0];
        let words = name.match(/[A-Z]?[a-z]*|[0-9]+|[-_]+/g);
        let result = words?.join(" ");
        return result?.trim() ?? name;
    }

    async function copy(path: string) {
        await navigator.clipboard.writeText(path);
        toast.success("Path copied to clipboard");
    }
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Asset Explorer</Card.Title>
        <Card.Description>All the assets avaliable to use in the playground.</Card.Description>
    </div>
    <Select.Root bind:selected={selectedFolder}>
        <Select.Trigger class="w-[200px]">
            <Select.Value class="capitalize" />
        </Select.Trigger>
        <Select.Content>
            <ScrollArea class="max-h-[calc(85vh-200px)]">
                {#each topLevelFolders as folder}
                    <Select.Item class="capitalize" value={folder}>
                        {folder.replace("_", " ")}
                    </Select.Item>
                {/each}
            </ScrollArea>
        </Select.Content>
    </Select.Root>
</Card.Header>
<Card.Content class="h-[calc(100%-90px)]">
    <ScrollArea class="h-full">
        <Table.Root>
            <Table.Body>
                {@const folderAssets = assetMap[selectedFolder.value]}
                {#each folderAssets as assetPath}
                    <Table.Row>
                        <Table.Cell
                            tabindex={0}
                            class="cursor-pointer focus:bg-accent focus:outline-none"
                            on:click={() => copy(assetPath)}
                            on:keydown={(e) => {
                                if (e.key === "Enter") copy(assetPath);
                            }}
                        >
                            <div class="font-medium capitalize">{getFileName(assetPath)}</div>
                            <div class="text-sm text-muted-foreground">
                                {assetPath}
                            </div>
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </ScrollArea>
</Card.Content>
