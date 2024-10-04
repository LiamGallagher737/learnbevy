<script lang="ts">
    import * as Tooltip from "$lib/components/ui/tooltip";
    interface Props {
        tooltip: string;
        children?: import("svelte").Snippet;
    }

    let { tooltip, children }: Props = $props();
</script>

<Tooltip.Root openDelay={0}>
    <Tooltip.Trigger>{@render children?.()}</Tooltip.Trigger>
    <Tooltip.Content>
        <p>{tooltip}</p>
    </Tooltip.Content>
</Tooltip.Root>
