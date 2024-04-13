<script lang="ts">
    import { tick } from "svelte";
    import { consoleItems, type LogLevel } from "./console";

    const logColors = {
        TRACE: "text-cyan-500",
        DEBUG: "text-yellow-500",
        INFO: "text-green-500",
        WARN: "text-orange-500",
        ERROR: "text-red-500",
    };

    let consoleElement: HTMLDivElement;

    let defaultConsoleLog = console.log;
    console.log = (...args) => {
        defaultConsoleLog.apply(console, args);
        const message: string = args[0];
        if (
            typeof message === "string" &&
            message?.startsWith("%c") &&
            !message?.includes("GPU lacks support")
        ) {
            const words = message.replaceAll("%c", "").split(" ");
            consoleItems.update((items) => [
                ...items,
                {
                    kind: "Log",
                    level: words[0] as LogLevel,
                    location: words[1],
                    message: words.slice(2).join(" "),
                },
            ]);
            scrollToBottomAfterTick();
        }
    };

    async function scrollToBottomAfterTick() {
        await tick();
        consoleElement.scroll({ top: consoleElement.scrollHeight, behavior: "smooth" });
    }
</script>

<div bind:this={consoleElement} class="h-full overflow-auto">
    {#each $consoleItems as item, n (n)}
        {#if item.kind === "Stdout"}
            <pre>{item.text}</pre>
        {:else if item.kind === "Log"}
            <div>
                <span class={logColors[item.level]}>{item.level}</span>{" "}
                <span class="text-neutral-500">{item.location}</span>{" "}
                {item.message}
            </div>
        {/if}
    {/each}
</div>
