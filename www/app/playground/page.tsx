"use client";

import { CodeEditor } from "@/components/code-editor";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { DEFAULT_CODE } from "@/lib/constants";
import { run } from "@/lib/runCode";
import { Share, Copy } from "lucide-react";
import { useRef, useState } from "react";
import { toast } from "sonner";

type State = "default" | "loadingGame" | "playingGame";

export default function Playground() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const gameCanvas = useRef<HTMLCanvasElement | null>(null);
  const wasm = useRef<{ __exit: () => void } | null>(null);
  const [state, setState] = useState<State>("default");

  async function copyCodeToClipboard() {
    await navigator.clipboard.writeText(code);
    toast("Code copied to clipboard");
  }

  return (
    <main className="p-4 h-screen">
      <ResizablePanelGroup
        direction="horizontal"
        onLayout={() => {
          const canvas = gameCanvas.current;
          if (canvas) {
            const parent = canvas.parentElement!;
            canvas.style.width = `${parent.clientWidth}px`;
            canvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
          }
        }}
      >
        <ResizablePanel
          defaultSize={60}
          minSize={20}
          className="flex flex-col gap-4"
        >
          <Card className="p-4 flex flex-row justify-between">
            <Button
              className="transition"
              onClick={async () => {
                if (wasm.current) wasm.current.__exit();
                if (gameCanvas.current) gameCanvas.current.remove();
                setState("loadingGame");
                const result = await run(code, "gameCard");
                setState("playingGame");
                gameCanvas.current = result.gameCanvas;
                wasm.current = result.wasm;
              }}
              disabled={state === "loadingGame"}
            >
              Run
            </Button>
            <div className="flex flex-row gap-4">
              <Button
                variant="outline"
                size="icon"
                onClick={copyCodeToClipboard}
              >
                <Copy className="h-4 w-4" />
              </Button>
              <Button variant="outline" size="icon">
                <Share className="h-4 w-4" />
              </Button>
            </div>
          </Card>
          <Card className="p-4 h-full">
            <CodeEditor
              onChange={(code) => {
                console.log(code);
                setCode(code);
              }}
            ></CodeEditor>
          </Card>
        </ResizablePanel>
        <ResizableHandle withHandle className="mx-4" />
        <ResizablePanel
          defaultSize={40}
          minSize={20}
          className="flex flex-col gap-4"
        >
          <Card className="aspect-video">
            <div id="gameCard" className="w-full h-full"></div>
          </Card>
          <Card className="flex-grow p-4 text-sm"></Card>
        </ResizablePanel>
      </ResizablePanelGroup>
    </main>
  );
}
