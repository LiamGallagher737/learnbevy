<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import { settings } from "./Settings.svelte";
    import type { Version } from "$lib/versions";
    import LoaderCircle from "lucide-svelte/icons/loader-circle";

    type CratesResponse = {
        crates: { name: string; version: string }[];
    };

    async function fetchCrates(version: Version) {
        const url = `/api/${version}/crates`;
        const response = await fetch(url);
        const result = await response.json();
        return result as CratesResponse;
    }
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Crates</Card.Title>
        <Card.Description>All the crates available to use in the playground.</Card.Description>
    </div>
</Card.Header>
<Card.Content class="h-[calc(100%-90px)]">
    {#await fetchCrates($settings.version)}
        <LoaderCircle class="animate-spin" />
    {:then response}
        <ScrollArea class="h-full">
            <Table.Root>
                <Table.Body>
                    {#each response.crates as crate}
                        <Table.Row>
                            <Table.Cell class="cursor-pointer focus-within:bg-accent">
                                <a
                                    href={"https://crates.io/crates/" + crate.name}
                                    target="_blank"
                                    class="block focus:outline-none"
                                >
                                    <div class="font-medium">{crate.name}</div>
                                    <div class="text-sm text-muted-foreground">
                                        {crate.version}
                                    </div>
                                </a>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </ScrollArea>
    {/await}
</Card.Content>
