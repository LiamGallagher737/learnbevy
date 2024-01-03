"use client";

import React, { useEffect } from "react";
import Editor, { loader } from "@monaco-editor/react";

export function CodeEditor(props: { defaultValue: string, onChange: (code: string) => void }) {
  useEffect(() => {
    loader.init().then((monaco) => {
      monaco.editor.defineTheme("custom-theme", {
        base: "vs-dark",
        inherit: true,
        rules: [],
        colors: {
          "editor.background": "#1c1917",
        },
      });
    });
  });

  return (
    <Editor
      height="100%"
      defaultLanguage="rust"
      defaultValue={props.defaultValue}
      theme="custom-theme"
      options={{ minimap: { enabled: false } }}
      onChange={(code) => props.onChange(code!)}
    />
  );
}
