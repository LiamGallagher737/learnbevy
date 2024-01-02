"use client";

import React from "react";
import Editor, { loader } from "@monaco-editor/react";
import { DEFAULT_CODE } from "@/lib/constants";

export function CodeEditor() {
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

  return (
    <Editor
      height="100%"
      defaultLanguage="rust"
      defaultValue={DEFAULT_CODE}
      theme="custom-theme"
      options={{ minimap: { enabled: false } }}
    />
  );
}
