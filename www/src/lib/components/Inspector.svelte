<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as Table from "$lib/components/ui/table";
    import * as Accordion from "$lib/components/ui/accordion";
    import { Separator } from "$lib/components/ui/separator";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>Inspector</Card.Title>
        <Card.Description>All the entities currently in your app.</Card.Description>
    </div>
</Card.Header>

<Card.Content class="flex h-[calc(100%-90px)] flex-row gap-6">
    <ScrollArea class="h-full w-56">
        <Table.Root>
            <Table.Body>
                <Table.Row>
                    <Table.Cell
                        tabindex={0}
                        class="cursor-pointer focus:bg-accent focus:outline-none"
                    >
                        <div class="font-medium capitalize">Camera (2v1)</div>
                    </Table.Cell>
                </Table.Row>
                <Table.Row>
                    <Table.Cell
                        tabindex={0}
                        class="cursor-pointer focus:bg-accent focus:outline-none"
                    >
                        <div class="font-medium capitalize">Entity (3v1)</div>
                    </Table.Cell>
                </Table.Row>
            </Table.Body>
        </Table.Root>
    </ScrollArea>
    <Separator orientation="vertical" />
    <Accordion.Root class="grow" multiple>
        <Accordion.Item value="2v1-name">
            <Accordion.Trigger class="text-sm">Name</Accordion.Trigger>
            <Accordion.Content>
                <div class="grid grid-cols-[8rem,1fr] items-center gap-2 p-1">
                    <Label for="2v1-name-0" class="text-muted-foreground">Value</Label>
                    <Input type="text" value="Camera" />
                    <Label for="2v1-name-1" class="text-muted-foreground">Alt Name</Label>
                    <Input type="text" value="Cool Camera" />
                </div>
            </Accordion.Content>
        </Accordion.Item>
        <Accordion.Item value="2v1-transform">
            <Accordion.Trigger class="text-sm">Transform</Accordion.Trigger>
            <Accordion.Content>
                <div class="grid grid-cols-[8rem,1fr] items-center gap-2 p-1">
                    <Label for="2v1-transform-0" class="capitalize text-muted-foreground">
                        translation.x
                    </Label>
                    <Input type="number" value="7" />
                    <Label for="2v1-transform-0" class="capitalize text-muted-foreground">
                        translation.y
                    </Label>
                    <Input type="number" value="1.5" />
                    <Label for="2v1-transform-0" class="capitalize text-muted-foreground">
                        translation.z
                    </Label>
                    <Input type="number" value="6" />
                </div>
            </Accordion.Content>
        </Accordion.Item>
        <Accordion.Item value="2v1-visibility">
            <Accordion.Trigger class="text-sm">Visibility</Accordion.Trigger>
            <Accordion.Content>Camera</Accordion.Content>
        </Accordion.Item>
    </Accordion.Root>
</Card.Content>
