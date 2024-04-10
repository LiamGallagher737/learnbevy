<script lang="ts">
    import * as Tooltip from '$lib/components/ui/tooltip';
    export let tooltip: string;
</script>

<Tooltip.Root openDelay={0}>
    <Tooltip.Trigger><slot /></Tooltip.Trigger>
    <Tooltip.Content>
        <p>{tooltip}</p>
    </Tooltip.Content>
</Tooltip.Root>
