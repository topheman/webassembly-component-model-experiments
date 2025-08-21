import { Fragment } from "react";

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
        <Fragment key={index}>
          {"stdin" in entry && entry.stdin && !entry.allowHtml && (
            <pre
              className="bg-gray-50 whitespace-pre-wrap word-break-word"
              data-stdtype="stdin"
              data-std-index={index}
            >
              {entry.stdin}
            </pre>
          )}
          {"stdin" in entry && entry.stdin && entry.allowHtml && (
            <pre
              className="bg-gray-50 whitespace-pre-wrap word-break-word"
              data-stdtype="stdin"
              data-std-index={index}
              // biome-ignore lint/security/noDangerouslySetInnerHtml: need to pass `allowHtml:true` to the addReplHistoryEntry function - only used for adding links to source of plugins when loaded
              dangerouslySetInnerHTML={{ __html: entry.stdin }}
            />
          )}
          {"stdout" in entry && entry.stdout && (
            <pre
              className="bg-green-100 whitespace-pre-wrap before:content-[attr(data-status)] relative before:absolute before:right-0 before:top-0 word-break-word"
              data-status={entry.status === "success" ? "✅" : "❌"}
              data-stdtype="stdout"
              data-std-index={index}
            >
              {entry.stdout}
            </pre>
          )}
          {"stderr" in entry && entry.stderr && (
            <pre
              className="bg-red-100 whitespace-pre-wrap before:content-[attr(data-status)] relative before:absolute before:right-0 before:top-0 word-break-word"
              data-status={entry.status === "success" ? "✅" : "❌"}
              data-stdtype="stderr"
              data-std-index={index}
            >
              {entry.stderr}
            </pre>
          )}
        </Fragment>
      ))}
    </div>
  );
}
