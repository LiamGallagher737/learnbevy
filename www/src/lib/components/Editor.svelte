<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

    const MODE_ID = 'rusty';

    let editor: Monaco.editor.IStandaloneCodeEditor;
    let monaco: typeof Monaco;
    let editorContainer: HTMLElement;

    onMount(async () => {
        // Import our 'monaco.ts' file here
        // (onMount() will only be executed in the browser, which is what we want)
        monaco = (await import('../monaco')).default;
        let rm = await import('$lib/rust-monaco');
        let themeVsDarkPlus = rm.themeVsDarkPlus;
        let config = rm.config;
        let grammar = rm.grammar;

        monaco.editor.defineTheme('vscode-dark-plus', themeVsDarkPlus);
        monaco.languages.register({
            id: MODE_ID,
        });
        monaco.languages.onLanguage(MODE_ID, async () => {
            monaco.languages.setLanguageConfiguration(MODE_ID, config);
            monaco.languages.setMonarchTokensProvider(MODE_ID, grammar);
        });

        // Your monaco instance is ready, let's display some code!
        const editor = monaco.editor.create(editorContainer, {
            theme: 'vscode-dark-plus',
        });
        const model = monaco.editor.createModel(
            'fn main() {\n\tprintln!("Hello, world!");\n}\n',
            MODE_ID
        );

        editor.setModel(model);
    });

    onDestroy(() => {
        monaco?.editor.getModels().forEach((model) => model.dispose());
        editor?.dispose();
    });
</script>

<div>
    <div class="container" bind:this={editorContainer} />
</div>

<style>
    .container {
        width: 100%;
        height: 600px;
    }
</style>
