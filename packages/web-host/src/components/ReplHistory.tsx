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
        <pre key={index}>{JSON.stringify(entry, null, 2)}</pre>
      ))}
    </div>
  );
}
