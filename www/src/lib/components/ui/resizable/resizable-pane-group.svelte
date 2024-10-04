<script lang="ts">
    import * as ResizablePrimitive from "paneforge";
    import { cn } from "$lib/utils.js";

    type $$Props = ResizablePrimitive.PaneGroupProps;

    interface Props {
        [key: string]: any;
    }

    let {
        class: className = undefined,
        direction,
        paneGroup = $bindable(undefined),
        el = $bindable(undefined),
        children,
        ...rest
    }: Props = $props();
</script>

<ResizablePrimitive.PaneGroup
    bind:el
    bind:paneGroup
    {direction}
    class={cn("flex h-full w-full data-[direction=vertical]:flex-col", className)}
    {...rest}
>
    {@render children?.()}
</ResizablePrimitive.PaneGroup>
