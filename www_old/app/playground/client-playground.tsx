"use client";

import { Copy, Share, Paintbrush, Settings, Info } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { CodeEditor } from "@/components/code-editor";
import { Console, ConsoleItem, LogLevel } from "@/components/console";
import { useEffect, useRef, useState } from "react";
import { run } from "@/lib/runCode";
import { toast } from "sonner";
import { createShare } from "./create-share";
import { useRouter } from "next/navigation";
import { formatCode } from "./format";
import { editor } from "monaco-editor";
import { VERSIONS, Version } from "@/lib/versions";
import { CHANNELS, Channel } from "@/lib/channels";
import { BasicTooltip } from "@/components/basic-tooltip";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ImperativePanelHandle } from "react-resizable-panels";

type State = "default" | "loadingGame" | "playingGame";

export default function ClientPlayground(params: {
  code: string;
  version: Version;
  channel: Channel;
}) {
  const router = useRouter();
  const gameCanvas = useRef<HTMLCanvasElement | null>(null);
  const wasm = useRef<{ __exit: () => void } | null>(null);
  const [consoleOutput, setConsoleOutput] = useState<ConsoleItem[]>([]);
  const [version, setVersion] = useState<Version>(params.version);
  const [channel, setChannel] = useState<Channel>(params.channel);
  const [state, setState] = useState<State>("default");
  const [editor, setEditor] = useState<editor.IStandaloneCodeEditor | null>(
    null
  );
  const gamePanel = useRef<ImperativePanelHandle | null>(null);

  useEffect(() => {
    const originalConsoleLog = console.log;
    console.log = (...args) => {
      originalConsoleLog.apply(console, args);
      const message: string = args[0];
      if (
        typeof message === "string" &&
        message?.startsWith("%c") &&
        !message?.includes("GPU lacks support")
      ) {
        const words = message.replaceAll("%c", "").split(" ");
        setConsoleOutput((prev) => [
          ...prev,
          {
            kind: "Log",
            level: words[0] as LogLevel,
            location: words[1],
            message: words.slice(2).join(" "),
          },
        ]);
      }
    };
  }, []);

  function recalculateGameCanvasSize() {
    const canvas = gameCanvas.current;
    if (canvas) {
      const parent = canvas.parentElement;
      if (parent) {
        canvas.style.width = `${parent.clientWidth}px`;
        canvas.style.height = `${parent.clientWidth * (9 / 16)}px`;
      }
    }
  }

  async function play() {
    if (wasm.current) wasm.current.__exit();
    if (gameCanvas.current) gameCanvas.current.remove();
    gamePanel.current?.expand();
    setConsoleOutput([]);

    setState("loadingGame");

    toast.promise(run(editor!.getValue(), version, channel, "gameCard"), {
      loading: "Loading...",
      success: (result) => {
        setState("playingGame");
        gameCanvas.current = result.gameCanvas;
        wasm.current = result.wasm;
        setConsoleOutput([{ kind: "Stdout", text: result.stderr }]);
        return "Built successfully";
      },
      error: (error) => {
        setState("default");
        if (error.cause?.stderr) {
          setConsoleOutput([{ kind: "Stdout", text: error.cause.stderr }]);
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
    toast.promise(createShare(editor!.getValue(), version, channel), {
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
          setConsoleOutput((prev) => [
            ...prev,
            { kind: "Stdout", text: error.cause.stderr },
          ]);
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
      onLayout={recalculateGameCanvasSize}
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
            <BasicTooltip tooltip="Format">
              <Button variant="outline" size="icon" onClick={format}>
                <Paintbrush className="h-4 w-4" />
              </Button>
            </BasicTooltip>

            <BasicTooltip tooltip="Copy">
              <Button
                variant="outline"
                size="icon"
                onClick={copyCodeToClipboard}
              >
                <Copy className="h-4 w-4" />
              </Button>
            </BasicTooltip>

            <BasicTooltip tooltip="Share">
              <Button variant="outline" size="icon" onClick={share}>
                <Share className="h-4 w-4" />
              </Button>
            </BasicTooltip>

            <Popover>
              <PopoverTrigger asChild>
                <Button variant="outline" size="icon">
                  <Settings className="h-4 w-4" />
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-80">
                <div className="flex flex-col gap-4">
                  <div className="space-y-2">
                    <h4 className="font-medium leading-none">Settings</h4>
                    <p className="text-sm text-muted-foreground">
                      Set additional settings for the playground
                    </p>
                  </div>
                  <div className="flex justify-between items-center gap-4">
                    <Label>Version</Label>
                    <Select
                      onValueChange={(v) => setVersion(v as Version)}
                      defaultValue={version}
                    >
                      <SelectTrigger className="w-[160px]">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {VERSIONS.map((version) => (
                          <SelectItem value={version} key={version}>
                            {version}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="flex justify-between items-center gap-4">
                    <Label className="flex items-center">
                      Channel
                      <BasicTooltip tooltip="Nightly allows some experimental features for faster builds">
                        <Button variant="link" size="icon">
                          <Info className="h-4 w-4" />
                        </Button>
                      </BasicTooltip>
                    </Label>
                    <Select
                      onValueChange={(v) => setChannel(v as Channel)}
                      defaultValue={channel}
                    >
                      <SelectTrigger className="w-[160px] capitalize">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {CHANNELS.map((channel) => (
                          <SelectItem
                            value={channel}
                            key={channel}
                            className="capitalize"
                          >
                            {channel}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </PopoverContent>
            </Popover>
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
        collapsible
        ref={gamePanel}
        onExpand={() => setTimeout(recalculateGameCanvasSize, 10)}
      >
        <Card className="aspect-video">
          <div id="gameCard" className="w-full h-full"></div>
        </Card>

        <Card className="flex-grow p-4 text-sm overflow-auto">
          <Console items={consoleOutput}></Console>
        </Card>
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}