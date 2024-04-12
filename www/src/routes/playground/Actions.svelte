<script lang="ts">
    import Paintbrush from 'lucide-svelte/icons/paintbrush';
    import Copy from 'lucide-svelte/icons/copy';
    import Share from 'lucide-svelte/icons/share';
    import BasicTooltip from '$lib/components/BasicTooltip.svelte';
    import { Button } from '$lib/components/ui/button';
</script>

<div class="flex flex-row gap-4">
    <BasicTooltip tooltip="Format">
        <Button variant="outline" size="icon">
            <Paintbrush class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Copy">
        <Button variant="outline" size="icon">
            <Copy class="h-4 w-4" />
        </Button>
    </BasicTooltip>

    <BasicTooltip tooltip="Share">
        <Button variant="outline" size="icon">
            <Share class="h-4 w-4" />
        </Button>
    </BasicTooltip>
</div>
