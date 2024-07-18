<script lang="ts">
    import * as Card from "$lib/components/ui/card";
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>About</Card.Title>
    </div>
</Card.Header>
<Card.Content class="space-y-4 text-pretty">
    <section class="space-y-2">
        <h4 class="text-md text-base font-semibold">How does it work?</h4>
        <p class="text-muted-foreground">
            Every time you press the play button, the code in the editor along with your selected
            Bevy version and Rust channel are sent to my server. Once there some extra code is added
            and then it is compiled to wasm inside of a podman container. After its compiled,
            wasm-bindgen works it's magic on the output which is then sent back to your browser
            along with the generated javascript from wasm-bindgen and the stdout of the compile
            process.
        </p>
    </section>

    <section class="space-y-2">
        <h4 class="text-md text-base font-semibold">How can I contribute?</h4>
        <p class="text-muted-foreground">
            All the code is available publicly on GitHub at
            <a
                href="https://github.com/LiamGallagher737/learnbevy"
                class="text-primary underline"
                target="_blank"
            >
                LiamGallagher737/learnbevy</a
            >.
        </p>
        <p class="text-muted-foreground">
            Remember that contributing doesn't mean having to write any code, it can be as simple as
            suggesting a new crate to be added!
        </p>
    </section>
</Card.Content>
