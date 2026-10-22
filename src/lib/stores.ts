import { writable, derived, get } from 'svelte/store';
import type { Tab, HistoryEntry, Bookmark, SidebarPanel, Theme, DownloadItem } from './types';

// ── Tabs ─────────────────────────────────────────────────────────────────────

function createTabStore() {
  const { subscribe, update, set } = writable<Tab[]>([]);

  function makeTab(url = 'lume://newtab'): Tab {
    return {
      id: crypto.randomUUID(),
      url,
      title: url === 'lume://newtab' ? 'New Tab' : url,
      favicon: '',
      loading: url !== 'lume://newtab',
      canGoBack: false,
      canGoForward: false,
      isActive: false,
    };
  }

  return {
    subscribe,
    init() {
      const tab = makeTab();
      tab.isActive = true;
      set([tab]);
      return tab.id;
    },
    open(url?: string) {
      let newId = '';
      update(tabs => {
        const tab = makeTab(url);
        tab.isActive = true;
        newId = tab.id;
        return tabs.map(t => ({ ...t, isActive: false })).concat(tab);
      });
      return newId;
    },
    close(id: string) {
      update(tabs => {
        const idx = tabs.findIndex(t => t.id === id);
        const next = tabs.filter(t => t.id !== id);
        if (next.length === 0) return [{ ...makeTab(), isActive: true }];
        if (tabs[idx]?.isActive) {
          const ni = Math.min(idx, next.length - 1);
          next[ni].isActive = true;
        }
        return next;
      });
    },
    activate(id: string) {
      update(tabs => tabs.map(t => ({ ...t, isActive: t.id === id })));
    },
    update: update,
    patch(id: string, data: Partial<Tab>) {
      update(tabs => tabs.map(t => t.id === id ? { ...t, ...data } : t));
    },
  };
}

export const tabs = createTabStore();

export const activeTab = derived(tabs, $tabs => $tabs.find(t => t.isActive) ?? null);

// ── History ───────────────────────────────────────────────────────────────────

function createHistoryStore() {
  const { subscribe, update } = writable<HistoryEntry[]>([]);
  return {
    subscribe,
    add(entry: Omit<HistoryEntry, 'id' | 'visitedAt'>) {
      if (!entry.url || entry.url.startsWith('lume://')) return;
      update(h => [{
        ...entry,
        id: crypto.randomUUID(),
        visitedAt: Date.now(),
      }, ...h].slice(0, 500));
    },
    clear() { update(() => []); },
  };
}

export const history = createHistoryStore();

// ── Bookmarks ─────────────────────────────────────────────────────────────────

function createBookmarkStore() {
  const { subscribe, update } = writable<Bookmark[]>([]);
  return {
    subscribe,
    add(entry: Omit<Bookmark, 'id' | 'createdAt'>) {
      update(b => [{
        ...entry,
        id: crypto.randomUUID(),
        createdAt: Date.now(),
      }, ...b]);
    },
    remove(id: string) {
      update(b => b.filter(x => x.id !== id));
    },
    has(url: string) {
      return get({ subscribe }).some(b => b.url === url);
    },
  };
}

export const bookmarks = createBookmarkStore();

// ── Downloads ─────────────────────────────────────────────────────────────────

export const downloads = writable<DownloadItem[]>([]);

// ── UI State ──────────────────────────────────────────────────────────────────

export const sidebarPanel = writable<SidebarPanel>('none');
export const theme = writable<Theme>('dark');
export const addressBarFocused = writable(false);
export const findBarVisible = writable(false);
export const findQuery = writable('');
