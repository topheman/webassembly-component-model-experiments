import { create } from "zustand";

import type { ReplHistoryEntry } from "../types";

const MAX_HISTORY_LENGTH = 200;

/**
 * Handles the state of the repl - the history of commands and their results.
 * @param state
 * @param payload
 * @returns
 */
function replStateReducer(
  state: Array<ReplHistoryEntry>,
  payload: ReplHistoryEntry,
) {
  if (state.length >= MAX_HISTORY_LENGTH) {
    // remove the oldest entry
    return [...state.slice(1), payload];
  }
  return [...state, payload];
}

const useInnerReplHistory = create<{
  history: ReplHistoryEntry[];
  addEntry: (entry: ReplHistoryEntry) => void;
}>((set) => ({
  history: [],
  addEntry: (entry: ReplHistoryEntry) => {
    set((state: { history: ReplHistoryEntry[] }) => ({
      history: replStateReducer(state.history, entry),
    }));
  },
}));

export function useReplHistory() {
  const { addEntry, history } = useInnerReplHistory((state) => state);
  return {
    addEntry,
    history,
  };
}
