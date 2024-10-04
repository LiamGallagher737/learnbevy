<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command";
    import * as Popover from "$lib/components/ui/popover";
    import { Button } from "$lib/components/ui/button";
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";
    import type { Version } from "$lib/versions";
    import { settings } from "./Settings.svelte";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
    import { toast } from "svelte-sonner";
    import { editorCode } from "$lib/components/editor";

    let open = $state(false);
    let value = $state("");

    async function loadExampleData(version: Version) {
        if (version !== "main") {
            const result = (await import(`../../lib/examples/${version}.examples.ts`)).examples;
            examples = result;
        } else {
            const result = await fetch("/api/examples-main").then((res) => res.json());
            examples = result;
        }
    }

    async function loadExampleCode() {
        const promise: Promise<void> = new Promise(async (resolve, reject) => {
            const branch = $settings.version !== "main" ? `v${$settings.version}.0` : "main";
            const url = `https://raw.githubusercontent.com/bevyengine/bevy/${branch}/examples/${value}`;
            const response = await fetch(url);
            if (!response.ok) {
                reject();
                return;
            }
            const code = await response.text();
            editorCode.set(code);
            resolve();
        });
        toast.promise(promise, {
            loading: "Loading...",
            success: "Loaded example",
            error: "Failed to load example",
        });
    }

    let examples: { value: string; label: string }[] = $state([]);
    let selectedValue = $derived(
        examples.find((f) => f.value === value)?.label ?? "Select an example..."
    );

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
        {#await loadExampleData($settings.version)}
            <Button
                disabled
                builders={[builder]}
                variant="outline"
                role="combobox"
                class="w-[250px] justify-between"
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
                class="w-[250px] justify-between"
            >
                {selectedValue}
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {:catch}
            <Button
                disabled
                builders={[builder]}
                variant="outline"
                role="combobox"
                class="w-[250px] justify-between"
            >
                Error loading examples
                <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        {/await}
    </Popover.Trigger>
    <Popover.Content class="w-[250px] p-0">
        <Command.Root>
            <Command.Input placeholder="Search example..." />
            <Command.Empty>No examples found.</Command.Empty>
            <Command.Group>
                <ScrollArea class="h-[calc(100vh-200px)]">
                    {#each examples as example}
                        <Command.Item
                            value={example.value}
                            onSelect={(currentValue) => {
                                value = currentValue;
                                closeAndFocusTrigger(ids.trigger);
                                loadExampleCode();
                            }}
                        >
                            <Check
                                class={cn(
                                    "mr-2 h-4 w-4",
                                    value !== example.value && "text-transparent"
                                )}
                            />
                            {example.label}
                        </Command.Item>
                    {/each}
                </ScrollArea>
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
