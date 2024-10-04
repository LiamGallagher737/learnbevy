<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import { Separator } from "$lib/components/ui/separator";
    import { wasmBindings } from "$lib/play";
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import Plus from "lucide-svelte/icons/plus";
    import Trash from "lucide-svelte/icons/trash";
    import InspectorEntityList from "./InspectorEntityList.svelte";
    import InspectorComponentList from "./InspectorComponentList.svelte";

    let searchQuery = "";
    let selectedEntity: number | null = null;

    $: console.log(selectedEntity);

    let entityListKey = {};
    function refreshEntities() {
        entityListKey = {};
    }

    async function spawnEntity() {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/spawn", { components: {} });
        if (typeof result === "object" && "code" in result) throw Error(result.message);
        return result.get("entity") as number;
    }

    async function despawnEntity(entity: number) {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/destroy", { entity });
        if (typeof result === "object" && "code" in result) throw Error(result.message);
    }
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Inspector</Card.Title>
        <Card.Description>All the entities currently in your app.</Card.Description>
    </div>
</Card.Header>

<Card.Content class="flex h-[calc(100%-90px)] flex-row gap-6">
    <div class="flex flex-col gap-2">
        <Input bind:value={searchQuery} type="text" placeholder="Search" />
        <div class="grid grid-cols-2 gap-2">
            <Button
                variant="outline"
                on:click={async () => {
                    selectedEntity = await spawnEntity();
                    refreshEntities();
                }}
            >
                <Plus size={16} />
            </Button>
            <Button
                variant="outline"
                disabled={selectedEntity === null}
                on:click={async () => {
                    await despawnEntity(selectedEntity ?? -1);
                    selectedEntity = null;
                    refreshEntities();
                }}
            >
                <Trash size={14} />
            </Button>
        </div>
        {#key entityListKey}
            <InspectorEntityList bind:selected={selectedEntity} bind:filter={searchQuery} />
        {/key}
    </div>

    <Separator orientation="vertical" />

    {#if selectedEntity !== null}
        {#key selectedEntity}
            <InspectorComponentList bind:selectedEntity />
        {/key}
    {/if}
</Card.Content>
