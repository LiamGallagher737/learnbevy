<script lang="ts">
    import * as Accordion from "$lib/components/ui/accordion";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";

    export let id: string;
    export let value: any;
</script>

{#if typeof value === "object"}
    {#if value instanceof Map}
        <Accordion.Root class="flex grow flex-col gap-2" multiple>
            {#each value.entries() as [n, v]}
                {@const nestedId = `${id}-${n}`}
                {#if typeof v === "object"}
                    <Accordion.Item value={nestedId} class="order-last">
                        <Accordion.Trigger class="text-sm">{n}</Accordion.Trigger>
                        <Accordion.Content class="pl-4">
                            <div class="flex flex-row gap-2 p-1">
                                <svelte:self id={nestedId} value={v} />
                            </div>
                        </Accordion.Content>
                    </Accordion.Item>
                {:else}
                    <div class="grid min-h-10 grid-cols-[8rem,1fr] items-center gap-2">
                        <Label for={nestedId} class="w-32 text-muted-foreground">{n}</Label>
                        <svelte:self id={nestedId} value={v} />
                    </div>
                {/if}
            {/each}
        </Accordion.Root>
    {:else if Array.isArray(value)}
        <div>
            {#if value.length > 0}
                {#each value as item, i}
                    <svelte:self id={`${id}-${i}`} value={item} />
                {/each}
            {:else}
                <p class="text-muted-foreground">Empty</p>
            {/if}
        </div>
    {/if}
{:else if typeof value === "boolean"}
    <Checkbox name={id} checked={value} />
{:else if typeof value === "number" || typeof value === "bigint"}
    <Input name={id} type="number" {value} />
{:else if typeof value === "string"}
    <Input name={id} type="text" {value} />
{/if}
