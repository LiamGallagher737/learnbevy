<script lang="ts">
    import { wasmBindings } from "$lib/play";
    import { onMount } from "svelte";
    import { ScrollArea } from "../ui/scroll-area";
    import * as Table from "../ui/table";
    import { toast } from "svelte-sonner";

    interface Props {
        selected: number | null;
        filter: string;
    }

    let { selected = $bindable(null), filter = "" }: Props = $props();

    let entities: any[] | null = $state(null);

    onMount(async () => {
        if (!$wasmBindings) return;

        const result = await $wasmBindings.brpRequest("bevy/query", {
            data: {
                //option: ["bevy_core::name::Name"],
            },
        });

        if (typeof result === "object" && "code" in result) {
            toast.error("Failed to fetch entities");
            throw Error(result.message);
        }

        entities = result;
    });

    function formatEntityKey(entity: number) {
        const combined = BigInt(entity);
        let gen = Number(combined >> 32n);
        let index = Number(combined & 0xffffffffn);
        return `${index}v${gen}`;
    }
</script>

<ScrollArea class="w-56 grow">
    {#if entities !== null}
        <Table.Root>
            <Table.Body>
                {#each entities as entity}
                    {@const nameComponent = entity
                        .get("components")
                        ?.get("bevy_core::name::Name")
                        ?.get("name")}
                    {@const name = `${nameComponent ?? "Entity"} (${formatEntityKey(entity.get("entity"))})`}
                    {#if filter.length === 0 || name.toLowerCase().includes(filter.toLowerCase())}
                        <Table.Row>
                            <Table.Cell
                                tabindex={0}
                                on:click={() => (selected = entity.get("entity"))}
                                on:keydown={(e) => {
                                    if (e.key === "Enter") selected = entity.get("entity");
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
    {:else}
        Loading...
    {/if}
</ScrollArea>
