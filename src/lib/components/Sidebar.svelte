<script lang="ts">
  import { sidebarPanel, bookmarks, history } from '../stores';
  import { navigate } from '../browser';
  import { activeTab } from '../stores';
  import type { SidebarPanel } from '../types';

  function close() { sidebarPanel.set('none'); }

  function open(url: string) {
    if ($activeTab) navigate($activeTab.id, url);
    close();
  }

  function formatDate(ts: number) {
    const d = new Date(ts);
    return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
  }
</script>

{#if $sidebarPanel !== 'none'}
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2 class="sidebar-title">
        {$sidebarPanel === 'bookmarks' ? 'Bookmarks' : 'History'}
      </h2>
      <button class="close-btn" on:click={close} type="button" title="Close">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M3 3l10 10M13 3L3 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>

    <div class="sidebar-content">
      {#if $sidebarPanel === 'bookmarks'}
        {#if $bookmarks.length === 0}
          <div class="empty">
            <span class="empty-icon">⭐</span>
            <p>No bookmarks yet</p>
            <span class="empty-hint">Click the bookmark icon in the address bar to save a page.</span>
          </div>
        {:else}
          {#each $bookmarks as bm}
            <button class="entry" on:click={() => open(bm.url)} type="button">
              {#if bm.favicon}
                <img src={bm.favicon} alt="" width="16" height="16" class="entry-icon" />
              {:else}
                <span class="entry-icon-fallback">⭐</span>
              {/if}
              <div class="entry-text">
                <span class="entry-title">{bm.title || bm.url}</span>
                <span class="entry-sub">{bm.url}</span>
              </div>
              <button
                class="remove-btn"
                on:click|stopPropagation={() => bookmarks.remove(bm.id)}
                type="button"
                title="Remove"
              >×</button>
            </button>
          {/each}
        {/if}

      {:else if $sidebarPanel === 'history'}
        {#if $history.length === 0}
          <div class="empty">
            <span class="empty-icon">🕐</span>
            <p>No history yet</p>
          </div>
        {:else}
          <button class="clear-btn" on:click={() => history.clear()} type="button">
            Clear all history
          </button>
          {#each $history as entry}
            <button class="entry" on:click={() => open(entry.url)} type="button">
              {#if entry.favicon}
                <img src={entry.favicon} alt="" width="16" height="16" class="entry-icon" />
              {:else}
                <span class="entry-icon-fallback">🌐</span>
              {/if}
              <div class="entry-text">
                <span class="entry-title">{entry.title || entry.url}</span>
                <span class="entry-sub">{formatDate(entry.visitedAt)}</span>
              </div>
            </button>
          {/each}
        {/if}
      {/if}
    </div>
  </aside>
{/if}

<style>
  .sidebar {
    width: 280px;
    min-width: 280px;
    background: var(--bg-surface);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .sidebar-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    width: 26px;
    height: 26px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    transition: background 0.1s, color 0.1s;
  }

  .close-btn:hover { background: var(--bg-hover); color: var(--text-primary); }

  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 40px 16px;
    gap: 8px;
    color: var(--text-muted);
    text-align: center;
  }

  .empty-icon { font-size: 28px; }
  .empty p { font-size: 13px; color: var(--text-secondary); }
  .empty-hint { font-size: 11px; }

  .entry {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px;
    border-radius: var(--radius-sm);
    width: 100%;
    text-align: left;
    transition: background 0.1s;
  }

  .entry:hover { background: var(--bg-elevated); }

  .entry-icon { flex-shrink: 0; object-fit: contain; }
  .entry-icon-fallback { font-size: 14px; flex-shrink: 0; width: 16px; text-align: center; }

  .entry-text {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .entry-title {
    font-size: 12px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .entry-sub {
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .remove-btn {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    color: var(--text-muted);
    opacity: 0;
    transition: opacity 0.1s, background 0.1s, color 0.1s;
  }

  .entry:hover .remove-btn { opacity: 1; }
  .remove-btn:hover { background: var(--bg-active); color: #ef4444; }

  .clear-btn {
    width: 100%;
    padding: 6px 8px;
    border-radius: var(--radius-sm);
    font-size: 11px;
    color: #ef4444;
    text-align: left;
    margin-bottom: 4px;
    transition: background 0.1s;
  }

  .clear-btn:hover { background: rgba(239, 68, 68, 0.1); }
</style>
