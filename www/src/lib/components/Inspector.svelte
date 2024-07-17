<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Inspector</Card.Title>
        <Card.Description>All the entities currently in your app.</Card.Description>
    </div>
</Card.Header>

<Card.Content class="h-[calc(100%-90px)]">
    <ScrollArea class="h-full">
        <Table.Root>
            <Table.Body></Table.Body>
        </Table.Root>
    </ScrollArea>
</Card.Content>
