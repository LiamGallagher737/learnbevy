<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
    import { settings } from "./Settings.svelte";

    type CratesResponse = {
        crates: { name: string; version: string; }[];
    };

    async function fetchCrates() {
        const url = `/api/${$settings.version}/crates`;
        const response = await fetch(url);
        const result = await response.json();
        return result as CratesResponse;
    }
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Asset Explorer</Card.Title>
        <Card.Description>All the assets avaliable to use in the playground.</Card.Description>
    </div>
</Card.Header>
<Card.Content class="h-[calc(100%-90px)]">
    <ScrollArea class="h-full">
        <Table.Root>
            <Table.Body>
                {#await fetchCrates()}
                    loading
                {:then response}
                    {#each response.crates as crate}
                        <Table.Row>
                            <Table.Cell
                                tabindex={0}
                                class="cursor-pointer focus:bg-accent focus:outline-none"
                            >
                                <div class="font-medium capitalize">{crate.name}</div>
                                <div class="text-sm text-muted-foreground">
                                    {crate.version}
                                </div>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                {/await}
            </Table.Body>
        </Table.Root>
    </ScrollArea>
</Card.Content>
