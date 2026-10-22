import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { tabs, history, activeTab } from './stores';
import { get } from 'svelte/store';

export interface NavEvent {
  tab_id: string;
  url: string;
  title?: string;
  favicon?: string;
  can_go_back: boolean;
  can_go_forward: boolean;
}

export interface LoadEvent {
  tab_id: string;
  loading: boolean;
}

let listenersInitialised = false;

export async function initBrowser() {
  if (listenersInitialised) return;
  listenersInitialised = true;

  await listen<NavEvent>('lume://nav', ({ payload }) => {
    tabs.patch(payload.tab_id, {
      url: payload.url,
      title: payload.title ?? payload.url,
      favicon: payload.favicon ?? '',
      canGoBack: payload.can_go_back,
      canGoForward: payload.can_go_forward,
    });

    if (payload.url && !payload.url.startsWith('lume://')) {
      history.add({
        url: payload.url,
        title: payload.title ?? payload.url,
        favicon: payload.favicon ?? '',
      });
    }
  });

  await listen<LoadEvent>('lume://load', ({ payload }) => {
    tabs.patch(payload.tab_id, { loading: payload.loading });
  });
}

// ── Navigation commands ───────────────────────────────────────────────────────

export async function navigate(tabId: string, url: string) {
  const cleaned = sanitiseUrl(url);
  tabs.patch(tabId, { url: cleaned, loading: true });
  await invoke('navigate_tab', { tabId, url: cleaned });
}

export async function goBack(tabId: string) {
  await invoke('tab_back', { tabId });
}

export async function goForward(tabId: string) {
  await invoke('tab_forward', { tabId });
}

export async function reload(tabId: string) {
  await invoke('tab_reload', { tabId });
}

export async function stopLoad(tabId: string) {
  await invoke('tab_stop', { tabId });
}

export async function createTab(url?: string) {
  const id = tabs.open(url);
  if (url && url !== 'lume://newtab') {
    await invoke('create_tab', { tabId: id, url });
  }
  return id;
}

export async function closeTab(tabId: string) {
  await invoke('close_tab', { tabId });
  tabs.close(tabId);
}

export async function activateTab(tabId: string) {
  tabs.activate(tabId);
  await invoke('activate_tab', { tabId });
}

// ── Utilities ─────────────────────────────────────────────────────────────────

export function sanitiseUrl(raw: string): string {
  const trimmed = raw.trim();
  if (!trimmed) return 'lume://newtab';

  // Already a protocol
  if (/^[a-z][a-z0-9+.-]*:\/\//i.test(trimmed)) return trimmed;

  // Looks like a hostname (has a dot, no spaces)
  if (/^[^\s]+\.[^\s]+$/.test(trimmed)) return `https://${trimmed}`;

  // Search query
  const query = encodeURIComponent(trimmed);
  return `https://search.brave.com/search?q=${query}`;
}

export function formatUrl(url: string): string {
  try {
    const u = new URL(url);
    const display = `${u.hostname}${u.pathname === '/' ? '' : u.pathname}`;
    return display.replace(/^www\./, '');
  } catch {
    return url;
  }
}

export function isSecure(url: string): boolean {
  return url.startsWith('https://') || url.startsWith('lume://');
}
