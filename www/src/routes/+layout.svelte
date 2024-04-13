<script lang="ts">
    import { ModeWatcher } from "mode-watcher";
    import "../app.pcss";
    import { Toaster } from "$lib/components/ui/sonner";
</script>

<ModeWatcher defaultMode="dark" />
<Toaster />
<slot />
