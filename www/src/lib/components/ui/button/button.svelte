<script lang="ts">
    import { Button as ButtonPrimitive } from "bits-ui";
    import { type Events, type Props, buttonVariants } from "./index.js";
    import { cn } from "$lib/utils.js";

    type $$Props = Props;
    type $$Events = Events;

    interface Props_1 {
        [key: string]: any;
    }

    let {
        class: className = undefined,
        variant = "default",
        size = "default",
        builders = [],
        children,
        disabled = $bindable(),
        ...rest
    }: Props_1 = $props();
</script>

<ButtonPrimitive.Root
    {builders}
    class={cn(buttonVariants({ variant, size, className }))}
    type="button"
    {...rest}
    on:click
    on:keydown
>
    {@render children?.()}
</ButtonPrimitive.Root>
