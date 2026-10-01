export {};

import type { AgentRequest, CoordinatorState, OpenProject } from "./model";

declare global {
  interface Window {
    milagre: {
      getCurrentProject: () => Promise<OpenProject>;
      openProject: () => Promise<OpenProject | null>;
      saveProject: (projectPath: string, state: CoordinatorState) => Promise<void>;
      sendToAgent: (request: AgentRequest) => Promise<string>;
      cancelAgent: () => Promise<boolean>;
    };
  }
}
