<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";

    type $$Props = HTMLAttributes<HTMLTableCaptionElement>;

    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, children, ...rest }: Props = $props();
</script>

<caption class={cn("mt-4 text-sm text-muted-foreground", className)} {...rest}>
    {@render children?.()}
</caption>
