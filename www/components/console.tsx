import { useEffect, useRef } from "react";

export function Console(props: { logs: string | string[] }) {
  const consoleBottomRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (consoleBottomRef.current) {
      consoleBottomRef.current.scrollIntoView();
    }
  }, [props.logs]);

  if (Array.isArray(props.logs)) {
    return (
      <>
        <pre className="text-wrap">
          {props.logs.map((log, n) => {
            const words = log.split(" ");
            let color = "text-white";
            switch (words[0]) {
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
                <span className={color}>{words[0]}</span>{" "}
                <span className="text-neutral-500">{words[1]}</span>{" "}
                {words.slice(2).join(" ")}
              </div>
            );
          })}
          <div ref={consoleBottomRef}></div>
        </pre>
      </>
    );
  } else {
    return (
      <>
        <pre className="text-wrap">
          {props.logs}
          <div ref={consoleBottomRef}></div>
        </pre>
      </>
    );
  }
}
