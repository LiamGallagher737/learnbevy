<script lang="ts">
    import { Command as CommandPrimitive } from "cmdk-sv";
    import { cn } from "$lib/utils.js";

    type $$Props = CommandPrimitive.CommandProps;

    interface Props {
        [key: string]: any;
    }

    let {
        value = $bindable(undefined),
        class: className = undefined,
        children,
        ...rest
    }: Props = $props();
</script>

<CommandPrimitive.Root
    class={cn(
        "flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
        className
    )}
    bind:value
    {...rest}
>
    {@render children?.()}
</CommandPrimitive.Root>
