<script lang="ts">
    import { Command as CommandPrimitive } from "cmdk-sv";
    import { cn } from "$lib/utils.js";

    type $$Props = CommandPrimitive.SeparatorProps;
    interface Props {
        [key: string]: any;
    }

    let { class: className = undefined, ...rest }: Props = $props();
</script>

<CommandPrimitive.Separator class={cn("-mx-1 h-px bg-border", className)} {...rest} />
