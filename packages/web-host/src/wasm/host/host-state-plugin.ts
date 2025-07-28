import { getReplVars } from "./host-state";

export function getReplVar(key: string): string | undefined {
  return getReplVars().find((replVar) => replVar.key === key)?.value;
}
