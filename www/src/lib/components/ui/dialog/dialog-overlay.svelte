<script lang="ts">
    import { Dialog as DialogPrimitive } from "bits-ui";
    import { fade } from "svelte/transition";
    import { cn } from "$lib/utils.js";

    type $$Props = DialogPrimitive.OverlayProps;

    interface Props {
        [key: string]: any;
    }

    let {
        class: className = undefined,
        transition = fade,
        transitionConfig = {
            duration: 150,
        },
        ...rest
    }: Props = $props();
</script>

<DialogPrimitive.Overlay
    {transition}
    {transitionConfig}
    class={cn("fixed inset-0 z-50 bg-background/80 backdrop-blur-sm", className)}
    {...rest}
/>
