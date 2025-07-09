import type { ReplHistoryEntry } from "../types";
import { cn } from "../utils/css";

interface ReplHistoryProps extends React.HTMLAttributes<HTMLDivElement> {
  ref: React.RefObject<HTMLDivElement | null>;
  history: ReplHistoryEntry[];
}

export function ReplHistory({
  history,
  className,
  ref,
  ...props
}: ReplHistoryProps) {
  return (
    <div ref={ref} className={cn(className)} {...props}>
      {history.map((entry, index) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: no unique key
        <div key={index}>
          {"stdin" in entry && entry.stdin && (
            <pre
              className="bg-gray-50 whitespace-pre-wrap"
              data-stdtype="stdin"
            >
              {entry.stdin}
            </pre>
          )}
          {"stdout" in entry && entry.stdout && (
            <pre
              className="bg-green-100 whitespace-pre-wrap before:content-[attr(data-status)] relative before:absolute before:right-0 before:top-0"
              data-status={entry.status === "success" ? "✅" : "❌"}
              data-stdtype="stdout"
            >
              {entry.stdout}
            </pre>
          )}
          {"stderr" in entry && entry.stderr && (
            <pre
              className="bg-red-100 whitespace-pre-wrap before:content-[attr(data-status)] relative before:absolute before:right-0 before:top-0"
              data-status={entry.status === "success" ? "✅" : "❌"}
              data-stdtype="stderr"
            >
              {entry.stderr}
            </pre>
          )}
        </div>
      ))}
    </div>
  );
}
