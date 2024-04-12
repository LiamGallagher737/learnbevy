<script context="module" lang="ts">
    export type ConsoleItem = Stdout | ConsoleLog;
    type Stdout = {
        kind: 'Stdout';
        text: string;
    };
    type LogLevel = 'TRACE' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
    type ConsoleLog = {
        kind: 'Log';
        level: LogLevel;
        location: string;
        message: string;
    };
</script>

<script lang="ts">
    import { browser } from '$app/environment';

    export let consoleItems: ConsoleItem[] = [];

    const logColors = {
        TRACE: 'text-cyan-500',
        DEBUG: 'text-yellow-500',
        INFO: 'text-green-500',
        WARN: 'text-orange-500',
        ERROR: 'text-red-500',
    };

    let defaultConsoleLog = console.log;
    console.log = (...args) => {
        defaultConsoleLog.apply(console, args);
        const message: string = args[0];
        if (
            typeof message === 'string' &&
            message?.startsWith('%c') &&
            !message?.includes('GPU lacks support')
        ) {
            const words = message.replaceAll('%c', '').split(' ');
            consoleItems = [
                ...consoleItems,
                {
                    kind: 'Log',
                    level: words[0] as LogLevel,
                    location: words[1],
                    message: words.slice(2).join(' '),
                },
            ];
        }
    };

    let consoleElement: HTMLDivElement;

    $: if (browser && consoleItems)
        consoleElement?.scroll({ top: consoleElement.scrollHeight, behavior: 'smooth' });
</script>

<div bind:this={consoleElement}>
    {#each consoleItems as item, n (n)}
        {#if item.kind === 'Stdout'}
            <pre>{item.text}</pre>
        {:else if item.kind === 'Log'}
            <div>
                <span class={logColors[item.level]}>{item.level}</span>{' '}
                <span class="text-neutral-500">{item.location}</span>{' '}
                {item.message}
            </div>
        {/if}
    {/each}
</div>
