<script lang="ts">
    import Settings from 'lucide-svelte/icons/settings';
    import Info from 'lucide-svelte/icons/info';
    import BasicTooltip from '$lib/components/BasicTooltip.svelte';
    import { Button } from '$lib/components/ui/button';
    import { Label } from '$lib/components/ui/label';
    import * as Popover from '$lib/components/ui/popover';
    import * as Select from '$lib/components/ui/select';
    import { DEFAULT_VERSION, VERSIONS, type Version } from '$lib/versions';
    import { CHANNELS, DEFAULT_CHANNEL, type Channel } from '$lib/channels';

    let selectedVersion = { label: DEFAULT_VERSION, value: DEFAULT_VERSION };
    let selectedChannel = { label: DEFAULT_CHANNEL, value: DEFAULT_CHANNEL };

    export let settings = {
        version: selectedVersion.value as Version,
        channel: selectedChannel.value as Channel,
    };

    $: settings.version = selectedVersion.value;
    $: settings.channel = selectedChannel.value;
</script>

<Popover.Root>
    <Popover.Trigger asChild let:builder>
        <Button variant="outline" size="icon" builders={[builder]}>
            <Settings class="h-4 w-4" />
        </Button>
    </Popover.Trigger>
    <Popover.Content class="w-80">
        <div class="flex flex-col gap-4">
            <div class="space-y-2">
                <h4 class="font-medium leading-none">Settings</h4>
                <p class="text-sm text-muted-foreground">
                    Set additional settings for the playground
                </p>
            </div>
            <div class="flex items-center justify-between gap-4">
                <Label>Version</Label>
                <Select.Root bind:selected={selectedVersion}>
                    <Select.Trigger class="w-[160px]">
                        <Select.Value />
                    </Select.Trigger>
                    <Select.Content>
                        {#each VERSIONS as version}
                            <Select.Item value={version} label={version}>
                                {version}
                            </Select.Item>
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
            <div class="flex items-center justify-between gap-4">
                <Label class="flex items-center">
                    Channel
                    <BasicTooltip
                        tooltip="Nightly allows some experimental features for faster builds"
                    >
                        <Button variant="link" size="icon">
                            <Info class="h-4 w-4" />
                        </Button>
                    </BasicTooltip>
                </Label>
                <Select.Root bind:selected={selectedChannel}>
                    <Select.Trigger class="w-[160px] capitalize">
                        <Select.Value />
                    </Select.Trigger>
                    <Select.Content>
                        {#each CHANNELS as channel}
                            <Select.Item value={channel} label={channel} class="capitalize">
                                {channel}
                            </Select.Item>
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>
        </div>
    </Popover.Content>
</Popover.Root>
