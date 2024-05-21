<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import "../app.pcss";
    import { Toaster } from "$lib/components/ui/sonner";
</script>

<svelte:head>
    <link rel="me" href="https://mastodon.social/@liam737" />
</svelte:head>

<ModeWatcher defaultMode="dark" />
<Toaster />
<slot />
