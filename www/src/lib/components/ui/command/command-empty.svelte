<script lang="ts">
    import { Command as CommandPrimitive } from "cmdk-sv";
    import { cn } from "$lib/utils.js";

    type $$Props = CommandPrimitive.EmptyProps;
    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, children, ...rest }: Props = $props();
</script>

<CommandPrimitive.Empty class={cn("py-6 text-center text-sm", className)} {...rest}>
    {@render children?.()}
</CommandPrimitive.Empty>
