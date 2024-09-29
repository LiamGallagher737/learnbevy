<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import * as Accordion from "$lib/components/ui/accordion";
    import { Separator } from "$lib/components/ui/separator";
    import { Input } from "$lib/components/ui/input";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
    import { wasmBindings } from "$lib/play";
    import InspectorValue from "./InspectorValue.svelte";

    let selectedEntity: number | null = null;

    async function getEntities() {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/query", {
            data: {
                //option: ["bevy_core::name::Name"],
            },
        });

        if ("code" in result) throw Error(result.message);
        return result;
    }

    async function getComponents(entity: number) {
        if (!$wasmBindings) throw Error("App is not running");

        const componentIds = await $wasmBindings.brpRequest("bevy/list", {
            entity,
        });
        if ("code" in componentIds) throw Error(componentIds.message);

        const components = await $wasmBindings.brpRequest("bevy/get", {
            entity,
            components: componentIds,
        });

        if ("code" in components) throw Error(components.message);

        let succeededComponentIds = Array.from(components.keys());
        let failedComponentIds = componentIds.filter(
            (comp: string) => !succeededComponentIds.includes(comp)
        );

        return [components, failedComponentIds];
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
    {#await getEntities() then entities}
        <div class="flex h-full flex-col gap-4">
            <!-- <Input type="text" placeholder="Search" /> -->
            <ScrollArea class="w-56 grow">
                <Table.Root>
                    <Table.Body>
                        {#each entities as entity}
                            {@const nameComponent = entity
                                .get("components")
                                ?.get("bevy_core::name::Name")
                                ?.get("name")}
                            {@const name = `${nameComponent ?? "Entity"} (${formatEntityKey(entity.get("entity"))})`}
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
                        {/each}
                    </Table.Body>
                </Table.Root>
            </ScrollArea>
        </div>
        <Separator orientation="vertical" />
        {#if selectedEntity !== null}
            {#await getComponents(selectedEntity) then [components, failedComponentIds]}
                <ScrollArea class="w-full">
                    <Accordion.Root class="mb-6 grow" multiple>
                        {#each components.entries() as [name, componentValue]}
                            <Accordion.Item value={`${selectedEntity}-${name}`}>
                                <Accordion.Trigger class="text-sm">
                                    {name.split("::").pop()}
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
