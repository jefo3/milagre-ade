const { contextBridge, ipcRenderer, webUtils } = require("electron");

contextBridge.exposeInMainWorld("milagre", {
  getPathForFile: (file) => webUtils.getPathForFile(file),
  listSkills: (projectPath) => ipcRenderer.invoke("skills:list", projectPath),
  listBranches: (projectPath) => ipcRenderer.invoke("project:branches", projectPath),
  getProjectImage: (projectPath) => ipcRenderer.invoke("project:image", projectPath),
  getAppVersion: () => ipcRenderer.invoke("app:version"),
  createWorktree: (request) => ipcRenderer.invoke("worktree:create", request),
  onWorktreeRenamed: (callback) => {
    const listener = (_event, rename) => callback(rename);
    ipcRenderer.on("worktree:renamed", listener);
    return () => ipcRenderer.removeListener("worktree:renamed", listener);
  },
  revealWorktree: (worktreePath) => ipcRenderer.invoke("worktree:reveal", worktreePath),
  getCurrentProject: () => ipcRenderer.invoke("project:current"),
  openProject: () => ipcRenderer.invoke("project:open"),
  readProject: (projectPath) => ipcRenderer.invoke("project:read", projectPath),
  onProjectState: (callback) => {
    const listener = (_event, update) => callback(update);
    ipcRenderer.on("project:state", listener);
    return () => ipcRenderer.removeListener("project:state", listener);
  },
  sendMessage: (request) => ipcRenderer.invoke("chat:send", request),
  patchChat: (projectPath, sessionId, patch) => ipcRenderer.invoke("chat:patch", projectPath, sessionId, patch),
  setOpenChat: (chatId) => ipcRenderer.invoke("chat:set-open", chatId),
  getRuns: () => ipcRenderer.invoke("chat:runs"),
  getModels: () => ipcRenderer.invoke("agent:models"),
  getCliStatus: () => ipcRenderer.invoke("agent:cli-status"),
  interruptAgent: (chatId) => ipcRenderer.invoke("agent:interrupt", chatId),
  respondToPermission: (chatId, requestId, decision) => ipcRenderer.invoke("agent:respond-permission", { chatId, requestId, decision }),
  answerQuestion: (chatId, requestId, answers) => ipcRenderer.invoke("agent:answer-question", { chatId, requestId, answers }),
  setAgentPermissionMode: (chatId, mode) => ipcRenderer.invoke("agent:set-permission-mode", { chatId, mode }),
  onAgentEvent: (callback) => {
    const listener = (_event, payload) => callback(payload);
    ipcRenderer.on("agent:event", listener);
    return () => ipcRenderer.removeListener("agent:event", listener);
  },
  getUpdateState: () => ipcRenderer.invoke("update:state"),
  installUpdate: () => ipcRenderer.invoke("update:install"),
  onUpdateState: (callback) => {
    const listener = (_event, state) => callback(state);
    ipcRenderer.on("update:state", listener);
    return () => ipcRenderer.removeListener("update:state", listener);
  },
  readUsage: () => ipcRenderer.invoke("usage:read"),
  getCachedUsage: () => ipcRenderer.invoke("usage:cached"),
  setNotifyWhenWaiting: (on) => ipcRenderer.invoke("settings:notify-when-waiting", on),
  onOpenChat: (callback) => {
    const listener = (_event, chatId) => callback(chatId);
    ipcRenderer.on("notification:open-chat", listener);
    return () => ipcRenderer.removeListener("notification:open-chat", listener);
  },
});
