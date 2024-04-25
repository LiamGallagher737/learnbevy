<script context="module" lang="ts">
    export type Tab = "editor" | "assets";
    export const selectedTab = writable<Tab>("editor");
</script>

<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";

    import Code from "lucide-svelte/icons/code";
    import Image from "lucide-svelte/icons/images";
    import { writable } from "svelte/store";
</script>

<div class="flex flex-col">
    <Button variant="ghost" class="h-12 rounded-b-none" on:click={() => selectedTab.set("editor")}>
        <Code />
    </Button>

    <Separator />

    <Button variant="ghost" class="h-12 rounded-none" on:click={() => selectedTab.set("assets")}>
        <Image />
    </Button>

    <Separator />
</div>
