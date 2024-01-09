"use client";

import { Copy, Share, Paintbrush } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { CodeEditor } from "@/components/code-editor";
import { Console } from "@/components/console";
import { useEffect, useRef, useState } from "react";
import { run } from "@/lib/runCode";
import { toast } from "sonner";
import { createShare } from "./create-share";
import { useRouter } from "next/navigation";
import { formatCode } from "./format";
import { editor } from "monaco-editor";
import { VersionPicker } from "./version-picker";
import { DEFAULT_VERSION, Version } from "@/lib/versions";

type State = "default" | "loadingGame" | "playingGame";

export default function ClientPlayground(params: { code: string }) {
  const router = useRouter();
  const gameCanvas = useRef<HTMLCanvasElement | null>(null);
  const wasm = useRef<{ __exit: () => void } | null>(null);
  const [consoleOutput, setConsoleOutput] = useState<string[]>([]);
  const [toolStderr, setToolStderr] = useState<string | null>(null);
  const version = useRef<Version>(DEFAULT_VERSION);
  const [state, setState] = useState<State>("default");
  const [editor, setEditor] = useState<editor.IStandaloneCodeEditor | null>(
    null
  );

  useEffect(() => {
    const originalConsoleLog = console.log;
    console.log = (...args) => {
      originalConsoleLog.apply(console, args);
      const message: string = args[0];
      if (
        message?.startsWith("%c") &&
        !message?.includes("GPU lacks support")
      ) {
        setConsoleOutput((prev) => [...prev, message.replaceAll("%c", "")]);
      }
    };
  }, []);

  async function play() {
    if (wasm.current) wasm.current.__exit();
    if (gameCanvas.current) gameCanvas.current.remove();
    setConsoleOutput([]);
    setToolStderr(null);

    setState("loadingGame");

    toast.promise(run(editor!.getValue(), version.current, "gameCard"), {
      loading: "Loading...",
      success: (result) => {
        setState("playingGame");
        gameCanvas.current = result.gameCanvas;
        wasm.current = result.wasm;
        setConsoleOutput([result.stderr]);
        return "Built successfully";
      },
      error: (error) => {
        setState("default");
        if (error.cause?.stderr) {
          setToolStderr(error.cause.stderr);
        }
        return error.message;
      },
    });
  }

  async function copyCodeToClipboard() {
    await navigator.clipboard.writeText(editor!.getValue());
    toast.success("Code copied to clipboard");
  }

  async function share() {
    toast.promise(createShare(editor!.getValue(), version.current), {
      loading: "Loading...",
      success: async ({ id }) => {
        await navigator.clipboard.writeText(
          `https://learnbevy.com/playground?share=${id}`
        );
        router.replace(`/playground?share=${id}`);
        return "Share link copied to clipboard";
      },
      error: "Error creating share link",
    });
  }

  async function format() {
    setToolStderr(null);
    const fmt = async (code: string) => {
      let response = await formatCode(code);
      if (response.kind === "Success") {
        return response.formatted_code;
      } else if (response.kind === "UserError") {
        throw new Error("Code couldn't be formatted", {
          cause: {
            source: "User",
            stderr: response.stderr.replace("<stdin>", "main.rs"),
          },
        });
      } else {
        throw new Error("A server error occurred", {
          cause: { source: "Server" },
        });
      }
    };
    toast.promise(fmt(editor!.getValue()), {
      loading: "Formatting...",
      success: async (formatted_code) => {
        editor?.setValue(formatted_code);
        return "Formatted successfully";
      },
      error: (error) => {
        if (error.cause.source === "User") {
          setToolStderr(error.cause.stderr);
          return "Your code could not be formatted";
        } else {
          return "Something went wrong...";
        }
      },
    });
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
          </div>

          <div className="flex flex-row gap-4">
            <VersionPicker
              initialValue={version.current}
              onChange={(v) => (version.current = v)}
            />
            <Button variant="outline" size="icon" onClick={format}>
              <Paintbrush className="h-4 w-4" />
            </Button>
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
            setEditor={setEditor}
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
          <Console logs={toolStderr ?? consoleOutput}></Console>
        </Card>
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}
