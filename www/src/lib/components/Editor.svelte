<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { DEFAULT_CODE } from '$lib/default-code';
    import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';

    const MODE_ID = 'rusty';

    let editor: Monaco.editor.IStandaloneCodeEditor;
    let monaco: typeof Monaco;
    let editorContainer: HTMLElement;

    export let code = DEFAULT_CODE;
    export const layout = () => {
        editor?.layout(); // Recalculates width and height
    };

    onMount(async () => {
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

        editor = monaco.editor.create(editorContainer, {
            theme: 'vscode-dark-plus',
            minimap: { enabled: false },
        });
        const model = monaco.editor.createModel(code, MODE_ID);

        editor.onDidChangeModelContent(() => (code = editor.getValue()));

        editor.setModel(model);
    });

    onMount(() => {
        window.addEventListener('resize', layout);
    });

    onDestroy(() => {
        monaco?.editor.getModels().forEach((model) => model.dispose());
        editor?.dispose();
    });
</script>

<div class="h-full w-full" bind:this={editorContainer} />
