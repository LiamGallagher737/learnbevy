<script lang="ts">
    import { wasmBindings } from "$lib/play";
    import { onDestroy, onMount } from "svelte";
    import * as Accordion from "../ui/accordion";
    import * as Card from "../ui/card";
    import { Button } from "../ui/button";
    import { ScrollArea } from "../ui/scroll-area";
    import InspectorValue from "./InspectorValue.svelte";
    import Trash from "lucide-svelte/icons/trash";
    import { SvelteMap, SvelteSet } from "svelte/reactivity";

    interface Props {
        selectedEntity: number | null;
    }

    let { selectedEntity = null }: Props = $props();

    let components = $state(new SvelteMap<string, any>());
    let failedComponentIds = $state(new SvelteSet<string>());
    let stopWatching = false;

    $inspect(components);

    onMount(async () => {
        if (!$wasmBindings) return;

        const componentIds = await $wasmBindings.brpRequest("bevy/list", {
            entity: selectedEntity,
        });

        if (typeof componentIds === "object" && "code" in componentIds)
            throw Error(componentIds.message);

        const initialRequest = await $wasmBindings.brpRequest("bevy/get", {
            entity: selectedEntity,
            components: componentIds,
        });

        if (typeof initialRequest === "object" && "code" in initialRequest)
            throw Error(initialRequest.message);

        components = new SvelteMap(initialRequest.get("components"));
        failedComponentIds = new SvelteSet(Array.from(initialRequest.get("errors").keys()));

        const stream = await $wasmBindings.brpStreamingRequest("bevy/get+watch", {
            entity: selectedEntity,
            components: componentIds,
        });

        const streamIterator = (async function* () {
            while (true) {
                const result = await stream.next();
                if (typeof result === "object" && "code" in result) yield undefined;
                else yield result;
                if (stopWatching) return { done: true };
            }
        })();

        watchComponentChanges(streamIterator);
    });

    async function watchComponentChanges(stream: AsyncGenerator<any>) {
        for await (const event of stream) {
            if (typeof event === "object" && "done" in event && event.done) break;
            if (event !== undefined) {
                event.get("components").forEach((value: any, key: string) => {
                    if (key === "bevy_render::view::visibility::ViewVisibility") return;
                    components.set(key, value);
                });
                event.get("removed").forEach((key: string) => {
                    components.delete(key);
                    failedComponentIds.delete(key);
                });
                event.get("errors").forEach((_value: any, key: string) => {
                    failedComponentIds.add(key);
                });
            } else {
                console.log("undefined :/");
            }
        }
    }

    onDestroy(() => (stopWatching = true));

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
    <Accordion.Root class="mb-6 grow" multiple>
        {#each components as [name, componentValue]}
            <Accordion.Item value={`${selectedEntity}-${name}`}>
                <Accordion.Trigger>
                    <div class="flex w-full justify-between pr-2">
                        <span>
                            {name.split("::").pop()}
                        </span>
                        <Button
                            class="h-6"
                            variant="ghost"
                            onclick={async () => {
                                await removeComponent(name, selectedEntity ?? -1);
                                components.delete(name);
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

    {#if failedComponentIds.size > 0}
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
