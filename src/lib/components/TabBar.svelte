<script lang="ts">
  import { tabs, activeTab } from '../stores';
  import { createTab, closeTab, activateTab } from '../browser';
  import type { Tab } from '../types';

  async function handleNew() {
    await createTab();
  }

  async function handleClose(e: MouseEvent, id: string) {
    e.stopPropagation();
    await closeTab(id);
  }

  async function handleActivate(id: string) {
    await activateTab(id);
  }

  function getDisplayTitle(tab: Tab) {
    return tab.title && tab.title.length > 22
      ? tab.title.slice(0, 22) + '…'
      : tab.title || 'New Tab';
  }
</script>

<div class="tabbar" data-tauri-drag-region>
  <div class="tabs-scroll">
    {#each $tabs as tab (tab.id)}
      <button
        class="tab"
        class:active={tab.isActive}
        class:loading={tab.loading}
        on:click={() => handleActivate(tab.id)}
        title={tab.title}
        type="button"
      >
        {#if tab.favicon}
          <img class="favicon" src={tab.favicon} alt="" width="14" height="14" />
        {:else if tab.loading}
          <span class="spinner" aria-label="Loading"></span>
        {:else}
          <span class="globe">🌐</span>
        {/if}

        <span class="tab-title">{getDisplayTitle(tab)}</span>

        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <span
          class="close"
          role="button"
          tabindex="-1"
          aria-label="Close tab"
          on:click|stopPropagation={(e) => handleClose(e, tab.id)}
          on:keydown={(e) => e.key === 'Enter' && handleClose(e as unknown as MouseEvent, tab.id)}
        >×</span>
      </button>
    {/each}
  </div>

  <button class="new-tab" on:click={handleNew} title="New tab (Ctrl+T)" type="button">
    <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
      <path d="M7 1v12M1 7h12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
    </svg>
  </button>
</div>

<style>
  .tabbar {
    height: var(--tab-height);
    background: var(--bg-base);
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 8px;
    border-bottom: 1px solid var(--border);
    overflow: hidden;
    flex-shrink: 0;
  }

  .tabs-scroll {
    display: flex;
    align-items: center;
    gap: 2px;
    overflow-x: auto;
    overflow-y: hidden;
    flex: 1;
    scrollbar-width: none;
  }
  .tabs-scroll::-webkit-scrollbar { display: none; }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 28px;
    padding: 0 10px 0 8px;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 80px;
    max-width: 200px;
    position: relative;
    transition: background 0.12s, color 0.12s;
  }

  .tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .tab.active {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .favicon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    object-fit: contain;
  }

  .globe { font-size: 12px; line-height: 1; flex-shrink: 0; }

  .spinner {
    width: 12px;
    height: 12px;
    border: 1.5px solid var(--text-muted);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  .tab-title {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 12px;
  }

  .close {
    width: 16px;
    height: 16px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 14px;
    line-height: 1;
    flex-shrink: 0;
    opacity: 0;
    transition: opacity 0.1s, background 0.1s, color 0.1s;
    cursor: pointer;
  }

  .tab:hover .close,
  .tab.active .close {
    opacity: 1;
  }

  .close:hover {
    background: var(--bg-active);
    color: var(--text-primary);
  }

  .new-tab {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    flex-shrink: 0;
    transition: background 0.12s, color 0.12s;
  }

  .new-tab:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
</style>
