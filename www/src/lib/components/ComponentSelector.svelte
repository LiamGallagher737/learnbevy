<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command";
    import * as Popover from "$lib/components/ui/popover";
    import { Button } from "$lib/components/ui/button";
    import { wasmBindings } from "$lib/play";
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
    import { TypePath } from "$lib/disqualified";

    interface Props {
        value?: TypePath | null;
    }

    let { value = $bindable(null) }: Props = $props();

    let open = $state(false);
    let componentIds: TypePath[] = $state([]);

    async function loadComponentIds() {
        if (!$wasmBindings) throw Error("App is not running");
        const result = await $wasmBindings.brpRequest("bevy/list", undefined);
        if (typeof result === "object" && "code" in result) throw Error(result.message);
        componentIds = result.map((type: string) => new TypePath(type));
    }

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
    }
</script>

<Popover.Root bind:open let:ids>
    <Popover.Trigger asChild let:builder>
        {#await loadComponentIds()}
            <Button
                disabled
                builders={[builder]}
                variant="outline"
                role="combobox"
                class="w-[400px] justify-between"
            >
                Loading...
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {:then}
            <Button
                builders={[builder]}
                variant="outline"
                role="combobox"
                aria-expanded={open}
                class="w-[400px] justify-between"
            >
                {value?.short() ?? "Select a component..."}
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {:catch}
            <Button
                disabled
                builders={[builder]}
                variant="outline"
                role="combobox"
                class="w-[400px] justify-between"
            >
                Error loading components
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {/await}
    </Popover.Trigger>
    <Popover.Content class="w-[400px] p-0">
        <Command.Root>
            <Command.Input placeholder="Search example..." />
            <Command.Empty>No components found.</Command.Empty>
            <Command.Group>
                <ScrollArea class="h-[calc(50vh-110px)]">
                    {#each componentIds as componentId}
                        <Command.Item
                            value={componentId.full()}
                            onSelect={(currentValue) => {
                                value = new TypePath(currentValue);
                                closeAndFocusTrigger(ids.trigger);
                            }}
                        >
                            <Check
                                class={cn(
                                    "mr-2 h-4 w-4",
                                    value?.full() !== componentId.full() && "text-transparent"
                                )}
                            />
                            {componentId.short()}
                        </Command.Item>
                    {/each}
                </ScrollArea>
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
