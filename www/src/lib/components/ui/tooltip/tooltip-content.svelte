<script lang="ts">
    import { Tooltip as TooltipPrimitive } from "bits-ui";
    import { cn, flyAndScale } from "$lib/utils.js";

    type $$Props = TooltipPrimitive.ContentProps;

    interface Props {
        [key: string]: any;
    }

    let {
        class: className = undefined,
        sideOffset = 4,
        transition = flyAndScale,
        transitionConfig = {
            y: 8,
            duration: 150,
        },
        children,
        ...rest
    }: Props = $props();
</script>

<TooltipPrimitive.Content
    {transition}
    {transitionConfig}
    {sideOffset}
    class={cn(
        "z-50 overflow-hidden rounded-md border bg-popover px-3 py-1.5 text-sm text-popover-foreground shadow-md",
        className
    )}
    {...rest}
>
    {@render children?.()}
</TooltipPrimitive.Content>
