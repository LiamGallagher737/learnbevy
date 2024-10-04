<script lang="ts">
    import { Accordion as AccordionPrimitive } from "bits-ui";
    import { slide } from "svelte/transition";
    import { cn } from "$lib/utils.js";

    type $$Props = AccordionPrimitive.ContentProps;

    interface Props {
        [key: string]: any;
    }

    let {
        class: className = undefined,
        transition = slide,
        transitionConfig = {
            duration: 200,
        },
        children,
        ...rest
    }: Props = $props();
</script>

<AccordionPrimitive.Content
    class={cn("overflow-hidden text-sm transition-all", className)}
    {transition}
    {transitionConfig}
    {...rest}
>
    <div class="pb-4 pt-0">
        {@render children?.()}
    </div>
</AccordionPrimitive.Content>
