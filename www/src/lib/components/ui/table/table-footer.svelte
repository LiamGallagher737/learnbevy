<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";

    type $$Props = HTMLAttributes<HTMLTableSectionElement>;

    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, children, ...rest }: Props = $props();
</script>

<tfoot class={cn("bg-primary font-medium text-primary-foreground", className)} {...rest}>
    {@render children?.()}
</tfoot>
