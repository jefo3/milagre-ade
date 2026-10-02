import type { AgentSession, ChatMessage } from "../model";

/** What the mark at the left of a chat row shows; the first that applies wins. */
export type ChatMark = "waiting" | "running" | "unread" | "idle";

export function chatMark({ waiting, running, unread }: { waiting: boolean; running: boolean; unread: boolean }): ChatMark {
  if (waiting) return "waiting";
  if (running) return "running";
  if (unread) return "unread";
  return "idle";
}

/** The chat's name: the one the user gave it, else the first line of its first message. */
export function chatTitle(session: AgentSession, messages: ChatMessage[]): string {
  if (session.title?.trim()) return session.title.trim();
  const line = messages.find((message) => message.role !== "assistant" && message.body.trim())?.body.trim().split("\n")[0] ?? "";
  if (!line) return session.agent_name;
  return line.length > 60 ? `${line.slice(0, 57)}…` : line;
}

/** A line count in a few characters: 980, 2.1k, 14k, 2.1m. */
export function formatLineCount(count: number): string {
  if (count < 1000) return String(count);
  if (count < 1_000_000) return `${trimDecimal(count / 1000)}k`;
  return `${trimDecimal(count / 1_000_000)}m`;
}

function trimDecimal(value: number) {
  return value < 10 ? value.toFixed(1).replace(/\.0$/, "") : String(Math.round(value));
}

/** The folder name at the end of a path. */
export function folderName(path: string): string {
  return path.replace(/[\\/]+$/, "").split(/[\\/]/).pop() || path;
}
