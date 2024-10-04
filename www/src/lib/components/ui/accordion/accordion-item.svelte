<script lang="ts">
    import { Accordion as AccordionPrimitive } from "bits-ui";
    import { cn } from "$lib/utils.js";

    type $$Props = AccordionPrimitive.ItemProps;

    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, value, children, ...rest }: Props = $props();
</script>

<AccordionPrimitive.Item {value} class={cn("border-b", className)} {...rest}>
    {@render children?.()}
</AccordionPrimitive.Item>
