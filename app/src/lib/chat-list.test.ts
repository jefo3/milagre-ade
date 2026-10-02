import assert from "node:assert/strict";
import test from "node:test";
import type { AgentSession, ChatMessage } from "../model";
import { chatMark, chatTitle, folderName, formatLineCount } from "./chat-list.ts";

const session: AgentSession = { id: 1, worktree_id: 1, agent_name: "main", status: "Created" };

test("chatMark: waiting beats running beats unread", () => {
  assert.equal(chatMark({ waiting: true, running: true, unread: true }), "waiting");
  assert.equal(chatMark({ waiting: false, running: true, unread: true }), "running");
  assert.equal(chatMark({ waiting: false, running: false, unread: true }), "unread");
  assert.equal(chatMark({ waiting: false, running: false, unread: false }), "idle");
});

test("chatTitle prefers the user's name, then the first message's first line", () => {
  const messages = [
    { id: 1, session_id: 1, body: "", context: null, role: "user" },
    { id: 2, session_id: 1, body: "Fix the login\nand more", context: null, role: "user" },
  ] as ChatMessage[];
  assert.equal(chatTitle(session, messages), "Fix the login");
  assert.equal(chatTitle({ ...session, title: "  Login bug " }, messages), "Login bug");
  assert.equal(chatTitle({ ...session, title: "   " }, messages), "Fix the login");
  assert.equal(chatTitle(session, []), "main");
  assert.equal(chatTitle(session, [{ id: 3, session_id: 1, body: "x".repeat(80), context: null }]), `${"x".repeat(57)}…`);
});

test("formatLineCount and folderName", () => {
  assert.equal(formatLineCount(0), "0");
  assert.equal(formatLineCount(980), "980");
  assert.equal(formatLineCount(1000), "1k");
  assert.equal(formatLineCount(2140), "2.1k");
  assert.equal(formatLineCount(14_400), "14k");
  assert.equal(formatLineCount(2_100_000), "2.1m");
  assert.equal(folderName("/Users/v/.milagre/worktrees/app/fix-login-ab12/"), "fix-login-ab12");
  assert.equal(folderName("/"), "/");
});
