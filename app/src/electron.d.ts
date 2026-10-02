export {};

import type { AgentRuns } from "./lib/agent-runs";
import type { SessionPatch, WorktreeRename } from "../../electron/shared/project-edits.mjs";
import type { AgentCliStatus, AgentModels, AgentEvent, ChatSendRequest, CoordinatorState, OpenProject, PermissionDecision, PermissionMode, QuestionAnswers, SkillCatalog, UsageSnapshot, WorktreeRequest } from "./model";

export type UpdateState = { status: "idle" | "checking" | "up-to-date" | "downloading" | "downloaded" | "error"; version: string | null; progress: number };

declare global {
  interface Window {
    milagre: {
      getPathForFile: (file: File) => string;
      listSkills: (projectPath: string) => Promise<SkillCatalog>;
      listBranches: (projectPath: string) => Promise<string[]>;
      getProjectImage: (projectPath: string) => Promise<string | null>;
      getAppVersion: () => Promise<string>;
      createWorktree: (request: WorktreeRequest) => Promise<{ project: OpenProject & { state: CoordinatorState }; worktreeId: number }>;
      /** A new worktree's branch got the name picked for its chat, a few seconds after it was created. */
      onWorktreeRenamed: (callback: (rename: WorktreeRename) => void) => () => void;
      /** Opens the worktree's folder in Finder. */
      revealWorktree: (worktreePath: string) => Promise<void>;
      getCurrentProject: () => Promise<OpenProject>;
      openProject: () => Promise<OpenProject | null>;
      /** Reads a project opened before in this run again, or null for any other path. */
      readProject: (projectPath: string) => Promise<OpenProject | null>;
      /** A project's state changed in the main process, its only writer. Changes made by agent events come with the event instead. */
      onProjectState: (callback: (update: { path: string; state: CoordinatorState }) => void) => () => void;
      /** Saves a message in its chat (a new one when `sessionId` is null), then starts or steers the chat's turn. */
      sendMessage: (request: ChatSendRequest) => Promise<{ sessionId: number }>;
      patchChat: (projectPath: string, sessionId: number, patch: SessionPatch) => Promise<void>;
      /** The chat on screen, by chat key, which opening reads; a turn that ends in any other chat leaves it unread. */
      setOpenChat: (chatId: string | null) => Promise<void>;
      /** The turns streaming now, in every project, and the number of the last agent event they hold. */
      getRuns: () => Promise<{ runs: AgentRuns; seq: number }>;
      respondToPermission: (chatId: string, requestId: string, decision: PermissionDecision) => Promise<boolean>;
      /** Sends the answers to a question card, or dismisses it (null). False when the question is gone. */
      answerQuestion: (chatId: string, requestId: string, answers: QuestionAnswers | null) => Promise<boolean>;
      setAgentPermissionMode: (chatId: string, mode: PermissionMode) => Promise<void>;
      /** Each agent's model list as its CLI reports it, asked once per app run; null for an agent that couldn't be asked. */
      getModels: () => Promise<AgentModels>;
      /** How each agent's CLI stands (missing, outdated, broken, logged out, or ready); checked again on every call while it has a problem. */
      getCliStatus: () => Promise<AgentCliStatus>;
      interruptAgent: (chatId: string) => Promise<void>;
      /** An agent event, with its project's new state when the event changed it, and its number once it's folded into the main process's runs (see getRuns). */
      onAgentEvent: (callback: (payload: { chatId: string; event: AgentEvent; state?: CoordinatorState; seq?: number }) => void) => () => void;
      getUpdateState: () => Promise<UpdateState>;
      installUpdate: () => Promise<void>;
      onUpdateState: (callback: (state: UpdateState) => void) => () => void;
      readUsage: () => Promise<UsageSnapshot>;
      getCachedUsage: () => Promise<UsageSnapshot>;
      /** Whether a chat that waits on the user while Milagre is in the background gets a system notification. */
      setNotifyWhenWaiting: (on: boolean) => Promise<void>;
      /** A notification was clicked: the window is back, and the chat it was about should open. */
      onOpenChat: (callback: (chatId: string) => void) => () => void;
    };
  }
}
