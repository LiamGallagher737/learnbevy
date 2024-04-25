<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import * as Select from "$lib/components/ui/select";
    import { onMount } from "svelte";

    let selectedFolder = { label: "textures", value: "textures" };

    type AssetMap = {
        [name: string]: string[];
    };
    let assets: Promise<{ assetMap: AssetMap; topLevelFolders: string[] }>;
    onMount(async () => {
        // @ts-ignore
        // it works so idk why ts is complaining
        assets = import("../../lib/asset-explorer/asset-map");
    });
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Asset Explorer</Card.Title>
        <Card.Description>All the assets avaliable to use in the playground.</Card.Description>
    </div>
    {#await assets then a}
        <Select.Root bind:selected={selectedFolder}>
            <Select.Trigger class="w-[200px]">
                <Select.Value class="capitalize" />
            </Select.Trigger>
            <Select.Content>
                <ScrollArea class="max-h-[calc(85vh-200px)]">
                    {#each a.topLevelFolders as folder}
                        <Select.Item class="capitalize" value={folder}>
                            {folder.replace("_", " ")}
                        </Select.Item>
                    {/each}
                </ScrollArea>
            </Select.Content>
        </Select.Root>
    {/await}
</Card.Header>
<Card.Content>
    
</Card.Content>
