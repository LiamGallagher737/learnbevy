<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";

    import Code from "lucide-svelte/icons/code";
    import Image from "lucide-svelte/icons/images";
</script>

<div class="flex flex-col">
    <Button variant="ghost" class="h-12 rounded-b-none"><Code /></Button>
    <Separator />
    <Button variant="ghost" class="h-12 rounded-none"><Image /></Button>
    <Separator />
</div>
