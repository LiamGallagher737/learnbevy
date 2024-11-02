<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import { ScrollArea } from "$lib/components/ui/scroll-area";
</script>

<Card.Header class="flex flex-row justify-between">
    <div class="grid gap-2">
        <Card.Title>About</Card.Title>
    </div>
</Card.Header>
<Card.Content class="h-[calc(100%-60px)] text-pretty">
    <ScrollArea class="h-full">
        <div class="space-y-4">
            <section class="space-y-2">
                <h4 class="text-md text-base font-semibold">How does it work?</h4>
                <p class="pb-2 text-muted-foreground">
                    The backend is hosted on a VPS from <a
                        href="https://snakecrafthosting.com"
                        class="text-primary underline"
                        target="_blank">Snakecraft Hosting</a
                    > who very generously sponsored this project. The server compiles and runs tools
                    like clippy and bevy_lint on the code in your editor in docker containers. In the
                    case of compiling, the WASM is sent back to your browser and ran like any other Bevy
                    web game.
                </p>
                <a href="https://snakecrafthosting.com" alt="Snakecraft Hosting" target="_blank">
                    <img src="https://my.snakecrafthosting.com/assets/img/logo.png" />
                </a>
            </section>

            <section class="space-y-2">
                <h4 class="text-md text-base font-semibold">How does it work?</h4>
                <p class="text-muted-foreground">
                    Every time you press the play button, the code in the editor along with your
                    selected Bevy version and Rust channel are sent to my server. Once there some
                    extra code is added and then it is compiled to wasm inside of a podman
                    container. After its compiled, wasm-bindgen works it's magic on the output which
                    is then sent back to your browser along with the generated javascript from
                    wasm-bindgen and the stdout of the compile process.
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
                    >. Feel free to have a go at some of the open issues or work on a new feature to
                    improve the playground!
                </p>
            </section>
        </div>
    </ScrollArea>
</Card.Content>
