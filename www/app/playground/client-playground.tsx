"use client";

import { Copy, Share } from "lucide-react";
import { Button } from "../../components/ui/button";
import { Card } from "../../components/ui/card";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "../../components/ui/resizable";
import { CodeEditor } from "@/components/code-editor";
import { Console } from "@/components/console";
import { useEffect, useRef, useState } from "react";
import { run } from "@/lib/runCode";
import { toast } from "sonner";
import Image from "next/image";
import { createShare } from "./create-share";

type State = "default" | "loadingGame" | "playingGame";

export default function ClientPlayground(params: { code: string }) {
  const [code, setCode] = useState(params.code);
  const gameCanvas = useRef<HTMLCanvasElement | null>(null);
  const wasm = useRef<{ __exit: () => void } | null>(null);
  const [consoleOutput, setConsoleOutput] = useState<string[]>([]);
  const [state, setState] = useState<State>("default");

  useEffect(() => {
    const originalConsoleLog = console.log;
    console.log = (...args) => {
      originalConsoleLog.apply(console, args);
      if (
        args[0]?.startsWith("%c") &&
        !args[0]?.includes("GPU lacks support")
      ) {
        setConsoleOutput((prev) => [...prev, args[0].replaceAll("%c", "")]);
      }
    };
  }, []);

  async function play() {
    if (wasm.current) wasm.current.__exit();
    if (gameCanvas.current) gameCanvas.current.remove();
    setConsoleOutput([]);

    setState("loadingGame");
    const result = await run(code, "gameCard");

    if (result.status === "Success") {
      setState("playingGame");
      gameCanvas.current = result.gameCanvas;
      wasm.current = result.wasm;
    } else if (result.status === "Error") {
      setState("default");
      if (result.stderr) {
        setConsoleOutput([result.stderr]);
      }
    }
  }

  async function copyCodeToClipboard() {
    await navigator.clipboard.writeText(code);
    toast("Code copied to clipboard");
  }

  async function share() {
    const { id } = await createShare(code);
    await navigator.clipboard.writeText(
      `https://learnbevy.com/playground?share=${id}`
    );
    toast("Share link copied to clipboard");
  }

  return (
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
          <div className="flex flex-row gap-4">
            <Button
              className="transition"
              onClick={play}
              disabled={state === "loadingGame"}
            >
              Play
            </Button>
            {state === "loadingGame" && (
              <Image
                className="animate-spin-slow"
                src="/assets/bevy_bird_dark.png"
                alt="Bevy Bird"
                width={40}
                height={40}
              />
            )}
          </div>
          <div className="flex flex-row gap-4">
            <Button variant="outline" size="icon" onClick={copyCodeToClipboard}>
              <Copy className="h-4 w-4" />
            </Button>
            <Button variant="outline" size="icon" onClick={share}>
              <Share className="h-4 w-4" />
            </Button>
          </div>
        </Card>

        <Card className="p-4 h-full">
          <CodeEditor
            defaultValue={params.code}
            onChange={(code) => setCode(code)}
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

        <Card className="flex-grow p-4 text-sm overflow-auto">
          <Console logs={consoleOutput}></Console>
        </Card>
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}
