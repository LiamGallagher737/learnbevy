<script lang="ts">
    import { Accordion as AccordionPrimitive } from "bits-ui";
    import ChevronDown from "lucide-svelte/icons/chevron-down";
    import { cn } from "$lib/utils.js";

    type $$Props = AccordionPrimitive.TriggerProps;
    type $$Events = AccordionPrimitive.TriggerEvents;

    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, level = 3, children, ...rest }: Props = $props();
</script>

<AccordionPrimitive.Header {level} class="flex">
    <AccordionPrimitive.Trigger
        class={cn(
            "flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180",
            className
        )}
        {...rest}
        on:click
    >
        {@render children?.()}
        <ChevronDown class="h-4 w-4 transition-transform duration-200" />
    </AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
