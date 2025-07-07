import type { ReplHistoryEntry } from "../hooks/replLogic";
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
            <pre className="bg-gray-50 whitespace-pre-wrap">{entry.stdin}</pre>
          )}
          {"stdout" in entry && entry.stdout && (
            <pre
              className="bg-green-100 whitespace-pre-wrap before:content-[attr(data-status)]"
              data-status={entry.status === "success" ? "✅ " : "❌ "}
            >
              {entry.stdout}
            </pre>
          )}
          {"stderr" in entry && entry.stderr && (
            <pre
              className="bg-red-100 whitespace-pre-wrap before:content-[attr(data-status)]"
              data-status={entry.status === "success" ? "✅ " : "❌ "}
            >
              {entry.stderr}
            </pre>
          )}
        </div>
      ))}
    </div>
  );
}
