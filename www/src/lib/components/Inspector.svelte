<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import * as Accordion from "$lib/components/ui/accordion";
    import { Separator } from "$lib/components/ui/separator";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
    import { wasmBindings } from "$lib/play";
    import { Button } from "$lib/components/ui/button";
    import InspectorValue from "./InspectorValue.svelte";
    import { Input } from "$lib/components/ui/input";
    import Plus from "lucide-svelte/icons/plus";
    import Trash from "lucide-svelte/icons/trash";
    import ComponentSelector from "./ComponentSelector.svelte";

    let searchQuery = "";
    let selectedEntity: number | null = null;

    let entitiesPromise = getEntities();
    function refreshEntities() {
        entitiesPromise = getEntities();
    }

    let componentsPromise = (entity: number) => {
        return getComponents(entity);
    };
    function refreshComponents() {
        componentsPromise = (entity: number) => {
            return getComponents(entity);
        };
    }

    async function getEntities() {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/query", {
            data: {
                //option: ["bevy_core::name::Name"],
            },
        });

        if (typeof result === "object" && "code" in result) throw Error(result.message);
        return result;
    }

    async function getComponents(entity: number) {
        if (!$wasmBindings) throw Error("App is not running");

        const componentIds = await $wasmBindings.brpRequest("bevy/list", {
            entity,
        });
        if (typeof componentIds === "object" && "code" in componentIds)
            throw Error(componentIds.message);

        const components = await $wasmBindings.brpRequest("bevy/get", {
            entity,
            components: componentIds,
        });

        if (typeof components === "object" && "code" in components) throw Error(components.message);

        let succeededComponentIds = Array.from(components.keys());
        let failedComponentIds = componentIds.filter(
            (comp: string) => !succeededComponentIds.includes(comp)
        );

        return [components, failedComponentIds];
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

    async function removeComponent(component: string, entity: number) {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/remove", {
            entity,
            components: [component],
        });
        if (typeof result === "object" && "code" in result) throw Error(result.message);
    }

    function formatEntityKey(entity: number) {
        const combined = BigInt(entity);
        let gen = Number(combined >> 32n);
        let index = Number(combined & 0xffffffffn);
        return `${index}v${gen}`;
    }
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Inspector</Card.Title>
        <Card.Description>All the entities currently in your app.</Card.Description>
    </div>
</Card.Header>

<Card.Content class="flex h-[calc(100%-90px)] flex-row gap-6">
    {#await entitiesPromise then entities}
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
            <ScrollArea class="w-56 grow">
                <Table.Root>
                    <Table.Body>
                        {#each entities as entity}
                            {@const nameComponent = entity
                                .get("components")
                                ?.get("bevy_core::name::Name")
                                ?.get("name")}
                            {@const name = `${nameComponent ?? "Entity"} (${formatEntityKey(entity.get("entity"))})`}
                            {#if searchQuery.length === 0 || name
                                    .toLowerCase()
                                    .includes(searchQuery.toLowerCase())}
                                <Table.Row>
                                    <Table.Cell
                                        tabindex={0}
                                        on:click={() => (selectedEntity = entity.get("entity"))}
                                        on:keydown={(e) => {
                                            if (e.key === "Enter")
                                                selectedEntity = entity.get("entity");
                                        }}
                                        class="cursor-pointer focus:bg-accent focus:outline-none"
                                    >
                                        <div class="font-medium capitalize">
                                            {name}
                                        </div>
                                    </Table.Cell>
                                </Table.Row>
                            {/if}
                        {/each}
                    </Table.Body>
                </Table.Root>
            </ScrollArea>
        </div>

        <Separator orientation="vertical" />

        {#if selectedEntity !== null}
            {#await componentsPromise(selectedEntity) then [components, failedComponentIds]}
                {#if components.size === 0 && failedComponentIds.length === 0}
                    <Card.Description class="pt-6">Empty</Card.Description>
                {/if}

                <ScrollArea class="w-full">
                    <Accordion.Root class="mb-6 grow" multiple>
                        {#each components.entries() as [name, componentValue]}
                            <Accordion.Item value={`${selectedEntity}-${name}`}>
                                <Accordion.Trigger>
                                    <div class="flex w-full justify-between pr-2">
                                        <span>
                                            {name.split("::").pop()}
                                        </span>
                                        <Button
                                            class="h-6"
                                            variant="ghost"
                                            on:click={async () => {
                                                await removeComponent(name, selectedEntity ?? -1);
                                                refreshComponents();
                                            }}
                                        >
                                            <Trash size={14} />
                                        </Button>
                                    </div>
                                </Accordion.Trigger>
                                <Accordion.Content class="pl-4">
                                    <InspectorValue
                                        id={`${selectedEntity}-${name}`}
                                        value={componentValue}
                                    />
                                </Accordion.Content>
                            </Accordion.Item>
                        {/each}
                    </Accordion.Root>

                    {#if failedComponentIds.length > 0}
                        <Card.Description>
                            The following components could not be inspected:
                            <ul class="list-inside list-disc break-all">
                                {#each failedComponentIds as component}
                                    <li>
                                        {component}
                                    </li>
                                {/each}
                            </ul>
                        </Card.Description>
                    {/if}
                </ScrollArea>
            {:catch err}
                <p>{err}</p>
            {/await}
        {/if}
    {:catch err}
        <p>{err}</p>
    {/await}
</Card.Content>
