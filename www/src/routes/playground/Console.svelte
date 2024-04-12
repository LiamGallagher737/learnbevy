<script context="module" lang="ts">
    export type ConsoleItem = Stdout | ConsoleLog;
    type Stdout = {
        kind: 'Stdout';
        text: string;
    };
    export type LogLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
    type ConsoleLog = {
        kind: 'Log';
        level: LogLevel;
        location: string;
        message: string;
    };
</script>

<script lang="ts">
    import { browser } from "$app/environment";

    export let consoleItems: ConsoleItem[];

    const logColors = {
        TRACE: 'text-cyan-500',
        DEBUG: 'text-yellow-500',
        INFO: 'text-green-500',
        WARN: 'text-orange-500',
        ERROR: 'text-red-500',
    };

    let console: HTMLDivElement;

    $: if (browser && consoleItems) console?.scroll({ top: console.scrollHeight, behavior: 'smooth' });
</script>

<div bind:this={console}>
    {#each consoleItems as item, n (n)}
        {#if item.kind === 'Stdout'}
            <p>{item.text}</p>
        {:else if item.kind === 'Log'}
            <div>
                <span class={logColors[item.level]}>{item.level}</span>{' '}
                <span class="text-neutral-500">{item.location}</span>{' '}
                {item.message}
            </div>
        {/if}
    {/each}
</div>
