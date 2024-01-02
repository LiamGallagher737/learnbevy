import { CodeEditor } from "@/components/code-editor";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Share, Copy } from "lucide-react";

export default function Playground() {
  return (
    <main className="p-4 h-screen">
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel
          defaultSize={60}
          minSize={20}
          className="flex flex-col gap-4"
        >
          <Card className="p-4 flex flex-row justify-between">
            <Button>Run</Button>
            <div className="flex flex-row gap-4">
              <Button variant="outline" size="icon">
                <Copy className="h-4 w-4" />
              </Button>
              <Button variant="outline" size="icon">
                <Share className="h-4 w-4" />
              </Button>
            </div>
          </Card>
          <Card className="p-4 h-full">
            <CodeEditor></CodeEditor>
          </Card>
        </ResizablePanel>
        <ResizableHandle withHandle className="mx-4" />
        <ResizablePanel
          defaultSize={40}
          minSize={20}
          className="flex flex-col gap-4"
        >
          <Card className="aspect-video"></Card>
          <Card className="flex-grow"></Card>
        </ResizablePanel>
      </ResizablePanelGroup>
    </main>
  );
}
