"use client";

import React, { useEffect } from "react";
import Editor, { loader } from "@monaco-editor/react";
import { config, grammar, themeVsDarkPlus } from "@/lib/rustMonacoDef";

const MODE_ID = "rusty";

export function CodeEditor(props: {
  defaultValue: string;
  onChange: (code: string) => void;
}) {
  useEffect(() => {
    loader.init().then((monaco) => {
      monaco.editor.defineTheme("vscode-dark-plus", themeVsDarkPlus);
      monaco.languages.register({
        id: MODE_ID,
      });
      monaco.languages.onLanguage(MODE_ID, async () => {
        monaco.languages.setLanguageConfiguration(MODE_ID, config);
        monaco.languages.setMonarchTokensProvider(MODE_ID, grammar);
      });
    });
  });

  return (
    <Editor
      height="100%"
      defaultLanguage="rusty"
      defaultValue={props.defaultValue}
      theme="vscode-dark-plus"
      options={{
        minimap: { enabled: false },
        "semanticHighlighting.enabled": true,
      }}
      onChange={(code) => props.onChange(code!)}
    />
  );
}
