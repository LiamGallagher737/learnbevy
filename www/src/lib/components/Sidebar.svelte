<script context="module" lang="ts">
    export type Tab = "editor" | "assets" | "crates" | "about";
    export const selectedTab = writable<Tab>("editor");
</script>

<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import { writable } from "svelte/store";

    import Code from "lucide-svelte/icons/code";
    import Image from "lucide-svelte/icons/images";
    import Package from "lucide-svelte/icons/package";
    import Info from "lucide-svelte/icons/info";

    export let tabs: Tab[];
</script>

<div class="flex flex-col">
    {#each tabs as tab, n}
        <Button
            variant="ghost"
            class={`h-12 ${n === 0 ? "rounded-b-none" : "rounded-none"}`}
            on:click={() => selectedTab.set(tab)}
        >
            {#if tab === "editor"}
                <Code />
            {:else if tab === "assets"}
                <Image />
            {:else if tab === "crates"}
                <Package />
            {:else if tab === "about"}
                <Info />
            {/if}
        </Button>
        <Separator />
    {/each}
</div>
