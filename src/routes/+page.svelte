<script lang="ts">
  import { onMount } from 'svelte';
  import TabBar from '$lib/components/TabBar.svelte';
  import AddressBar from '$lib/components/AddressBar.svelte';
  import NewTabPage from '$lib/components/NewTabPage.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { tabs, activeTab, findBarVisible, findQuery, sidebarPanel } from '$lib/stores';
  import { initBrowser, createTab, navigate, goBack, goForward, reload } from '$lib/browser';

  onMount(async () => {
    await initBrowser();
    tabs.init();
  });

  // Keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    const ctrl = e.ctrlKey || e.metaKey;

    if (ctrl && e.key === 't') { e.preventDefault(); createTab(); }
    if (ctrl && e.key === 'w') { e.preventDefault(); if ($activeTab) import('$lib/browser').then(m => m.closeTab($activeTab!.id)); }
    if (ctrl && e.key === 'r') { e.preventDefault(); if ($activeTab) reload($activeTab.id); }
    if (ctrl && e.key === 'f') { e.preventDefault(); findBarVisible.update(v => !v); }
    if (ctrl && e.key === 'h') { e.preventDefault(); sidebarPanel.update(p => p === 'history' ? 'none' : 'history'); }
    if (ctrl && e.key === 'b') { e.preventDefault(); sidebarPanel.update(p => p === 'bookmarks' ? 'none' : 'bookmarks'); }
    if (e.key === 'F5') { e.preventDefault(); if ($activeTab) reload($activeTab.id); }
    if (e.altKey && e.key === 'ArrowLeft') { e.preventDefault(); if ($activeTab) goBack($activeTab.id); }
    if (e.altKey && e.key === 'ArrowRight') { e.preventDefault(); if ($activeTab) goForward($activeTab.id); }

    // Tab switching: Ctrl+1-9
    if (ctrl && e.key >= '1' && e.key <= '9') {
      e.preventDefault();
      const idx = parseInt(e.key) - 1;
      const tabList = $tabs;
      if (tabList[idx]) {
        import('$lib/browser').then(m => m.activateTab(tabList[idx].id));
      }
    }
  }

  $: isNewTab = $activeTab?.url === 'lume://newtab';
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="browser">
  <div class="chrome">
    <TabBar />
    <AddressBar />

    {#if $findBarVisible}
      <div class="findbar">
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="6" cy="6" r="4.5" stroke="currentColor" stroke-width="1.2"/>
          <path d="M9.5 9.5l3 3" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
        <input
          bind:value={$findQuery}
          class="find-input"
          type="text"
          placeholder="Find in page…"
          autofocus
          on:keydown={(e) => e.key === 'Escape' && findBarVisible.set(false)}
        />
        <button class="find-close" on:click={() => findBarVisible.set(false)} type="button">×</button>
      </div>
    {/if}
  </div>

  <div class="content-area">
    {#if isNewTab}
      <NewTabPage />
    {:else}
      <div class="webview-placeholder">
        <!-- Tauri child webview renders here — positioned by Rust -->
        <div class="loading-indicator" class:active={$activeTab?.loading}>
          <div class="progress-bar"></div>
        </div>
      </div>
    {/if}

    <Sidebar />
  </div>
</div>

<style>
  .browser {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .chrome {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    background: var(--bg-surface);
  }

  .findbar {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 12px;
    background: var(--bg-elevated);
    border-bottom: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .find-input {
    flex: 1;
    font-size: 13px;
    user-select: text;
  }

  .find-input::placeholder { color: var(--text-muted); }

  .find-close {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    font-size: 16px;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .find-close:hover { background: var(--bg-active); color: var(--text-primary); }

  .content-area {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .webview-placeholder {
    flex: 1;
    background: var(--bg-base);
    position: relative;
    overflow: hidden;
  }

  .loading-indicator {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: transparent;
    z-index: 100;
    overflow: hidden;
  }

  .loading-indicator.active .progress-bar {
    position: absolute;
    top: 0;
    left: -40%;
    width: 40%;
    height: 100%;
    background: linear-gradient(90deg, transparent, var(--accent), transparent);
    animation: progress 1.2s ease-in-out infinite;
  }

  @keyframes progress {
    0%   { left: -40%; }
    100% { left: 120%; }
  }
</style>
