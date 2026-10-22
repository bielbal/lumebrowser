export interface Tab {
  id: string;
  url: string;
  title: string;
  favicon: string;
  loading: boolean;
  canGoBack: boolean;
  canGoForward: boolean;
  isActive: boolean;
}

export interface HistoryEntry {
  id: string;
  url: string;
  title: string;
  visitedAt: number;
  favicon: string;
}

export interface Bookmark {
  id: string;
  url: string;
  title: string;
  favicon: string;
  createdAt: number;
  folder?: string;
}

export interface DownloadItem {
  id: string;
  url: string;
  filename: string;
  progress: number;
  state: 'downloading' | 'completed' | 'error';
  size?: number;
}

export type SidebarPanel = 'none' | 'bookmarks' | 'history' | 'downloads';
export type Theme = 'dark' | 'light' | 'system';
