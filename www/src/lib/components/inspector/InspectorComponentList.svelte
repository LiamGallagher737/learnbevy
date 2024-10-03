<script lang="ts">
    import { wasmBindings } from "$lib/play";
    import { onMount } from "svelte";
    import * as Accordion from "../ui/accordion";
    import * as Card from "../ui/card";
    import { Button } from "../ui/button";
    import { ScrollArea } from "../ui/scroll-area";
    import InspectorValue from "./InspectorValue.svelte";
    import Trash from "lucide-svelte/icons/trash";

    export let selectedEntity: number | null = null;

    let components: Map<string, any> | null = null;
    let failedComponentIds: string[] = [];

    onMount(async () => {
        if (!$wasmBindings) return;

        const stream = await $wasmBindings.brpStreamingRequest("bevy/list", {
            entity: selectedEntity,
        });

        // Temp solution until I find a way to add this from wasm-bindgen side
        stream[Symbol.asyncIterator] = function () {
            return this;
        };

        for await (const componentIds of stream) {
            console.log(componentIds);
            if (componentIds === undefined) continue;

            const result = await $wasmBindings.brpRequest("bevy/get", {
                entity: selectedEntity,
                components: componentIds,
            });

            if (typeof result === "object" && "code" in result) throw Error(result.message);
            components = result.get("components");
            failedComponentIds = Array.from(result.get("errors").keys());
        }
    });

    async function removeComponent(component: string, entity: number) {
        if (!$wasmBindings) throw Error("App is not running");

        const result = await $wasmBindings.brpRequest("bevy/remove", {
            entity,
            components: [component],
        });
        if (typeof result === "object" && "code" in result) throw Error(result.message);
    }
</script>

<ScrollArea class="w-full">
    {#if components !== null}
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
                                }}
                            >
                                <Trash size={14} />
                            </Button>
                        </div>
                    </Accordion.Trigger>
                    <Accordion.Content class="pl-4">
                        <InspectorValue id={`${selectedEntity}-${name}`} value={componentValue} />
                    </Accordion.Content>
                </Accordion.Item>
            {/each}
        </Accordion.Root>
    {:else}
        Loading...
    {/if}

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
