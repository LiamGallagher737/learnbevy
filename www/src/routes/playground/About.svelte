<script lang="ts">
    import * as Card from "$lib/components/ui/card";
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>About</Card.Title>
    </div>
</Card.Header>
<Card.Content class="text-pretty">
    <Card.Title tag="h4" class="text-base">How can I contribute?</Card.Title>
    <p class="pb-2 text-muted-foreground">
        All the code is avaliable publicly on GitHub at
        <a
            href="https://github.com/LiamGallagher737/learnbevy"
            class="text-primary underline"
            target="_blank"
        >
            LiamGallagher737/learnbevy</a
        >.
    </p>
    <p class="text-muted-foreground">
        Keep in mind that contributing doesn't mean having to write any code, it can be as simple as
        suggesting a new crate to be added!
    </p>
</Card.Content>
