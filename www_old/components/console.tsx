import { useEffect, useRef } from "react";

export function Console(props: { items: ConsoleItem[] }) {
  const consoleBottomRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (consoleBottomRef.current) {
      consoleBottomRef.current.scrollIntoView();
    }
  }, [props.items]);

  const content = props.items.map((item, n) => {
    if (item.kind === "Stdout") {
      return <p key={n}>{item.text}</p>;
    } else if (item.kind === "Log") {
      let color = null;
      switch (item.level) {
        case "TRACE":
          color = "text-cyan-500";
          break;
        case "DEBUG":
          color = "text-yellow-500";
          break;
        case "INFO":
          color = "text-green-500";
          break;
        case "WARN":
          color = "text-orange-500";
          break;
        case "ERROR":
          color = "text-red-500";
          break;
      }
      return (
        <div key={n}>
          <span className={color}>{item.level}</span>{" "}
          <span className="text-neutral-500">{item.location}</span>{" "}
          {item.message}
        </div>
      );
    }
  });

  return (
    <>
      <pre className="text-wrap">
        {content}
        <div ref={consoleBottomRef}></div>
      </pre>
    </>
  );
}

export type ConsoleItem = Stdout | ConsoleLog;

export type Stdout = {
  kind: "Stdout";
  text: string;
};

export type LogLevel = "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR";
export type ConsoleLog = {
  kind: "Log";
  level: LogLevel;
  location: string;
  message: string;
};
