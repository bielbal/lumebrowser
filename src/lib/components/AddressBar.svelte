<script lang="ts">
  import { activeTab, sidebarPanel, bookmarks as bmStore } from '../stores';
  import { navigate, goBack, goForward, reload, stopLoad, isSecure } from '../browser';

  let inputValue = '';
  let focused = false;
  let inputEl: HTMLInputElement;

  $: if ($activeTab) {
    if (!focused) {
      inputValue = $activeTab.url === 'lume://newtab' ? '' : $activeTab.url;
    }
  }

  $: isBookmarked = $activeTab ? bmStore.has($activeTab.url) : false;
  $: secure = $activeTab ? isSecure($activeTab.url) : false;
  $: loading = $activeTab?.loading ?? false;

  function handleFocus() {
    focused = true;
    inputValue = $activeTab?.url === 'lume://newtab' ? '' : ($activeTab?.url ?? '');
    setTimeout(() => inputEl?.select(), 0);
  }

  function handleBlur() {
    focused = false;
    if ($activeTab) {
      inputValue = $activeTab.url === 'lume://newtab' ? '' : $activeTab.url;
    }
  }

  async function handleSubmit() {
    if (!$activeTab || !inputValue.trim()) return;
    inputEl?.blur();
    await navigate($activeTab.id, inputValue);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      inputEl?.blur();
    }
  }

  function toggleBookmark() {
    if (!$activeTab || $activeTab.url === 'lume://newtab') return;
    if (isBookmarked) {
      const bm = $bmStore.find(b => b.url === $activeTab!.url);
      if (bm) bmStore.remove(bm.id);
    } else {
      bmStore.add({
        url: $activeTab.url,
        title: $activeTab.title,
        favicon: $activeTab.favicon,
      });
    }
  }

  function toggleSidebar(panel: 'bookmarks' | 'history') {
    sidebarPanel.update(p => p === panel ? 'none' : panel);
  }
</script>

<div class="navbar">
  <!-- Back / Forward / Reload -->
  <div class="nav-controls">
    <button
      class="icon-btn"
      disabled={!($activeTab?.canGoBack)}
      on:click={() => $activeTab && goBack($activeTab.id)}
      title="Back (Alt+←)"
      type="button"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M10 3L5 8l5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <button
      class="icon-btn"
      disabled={!($activeTab?.canGoForward)}
      on:click={() => $activeTab && goForward($activeTab.id)}
      title="Forward (Alt+→)"
      type="button"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M6 3l5 5-5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <button
      class="icon-btn"
      on:click={() => $activeTab && (loading ? stopLoad($activeTab.id) : reload($activeTab.id))}
      title={loading ? 'Stop (Esc)' : 'Reload (Ctrl+R)'}
      type="button"
    >
      {#if loading}
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <rect x="5" y="5" width="6" height="6" rx="1" fill="currentColor"/>
        </svg>
      {:else}
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M13 8A5 5 0 1 1 8 3c1.5 0 2.8.66 3.73 1.7M13 3v3h-3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      {/if}
    </button>
  </div>

  <!-- Address input -->
  <div class="address-wrap" class:focused>
    <span class="security-icon" class:secure class:insecure={!secure && !($activeTab?.url.startsWith('lume://'))}>
      {#if $activeTab?.url.startsWith('lume://')}
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <path d="M7 1.5A5.5 5.5 0 1 0 12.5 7 5.5 5.5 0 0 0 7 1.5zM7 3.5V7l2 2" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      {:else if secure}
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <rect x="3" y="6" width="8" height="6" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
          <path d="M5 6V4.5a2 2 0 1 1 4 0V6" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="5.5" stroke="currentColor" stroke-width="1.2"/>
          <path d="M7 4v3.5M7 9.5v.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      {/if}
    </span>

    <input
      bind:this={inputEl}
      bind:value={inputValue}
      class="address-input"
      type="text"
      placeholder="Search or enter address"
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      spellcheck="false"
      on:focus={handleFocus}
      on:blur={handleBlur}
      on:keydown={handleKeydown}
      on:keydown={(e) => e.key === 'Enter' && handleSubmit()}
    />
  </div>

  <!-- Right actions -->
  <div class="right-actions">
    <button
      class="icon-btn"
      class:bookmarked={isBookmarked}
      on:click={toggleBookmark}
      title={isBookmarked ? 'Remove bookmark' : 'Add bookmark (Ctrl+D)'}
      type="button"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path
          d="M4 2.5h8a1 1 0 0 1 1 1V13l-5-3-5 3V3.5a1 1 0 0 1 1-1z"
          stroke={isBookmarked ? 'var(--accent)' : 'currentColor'}
          stroke-width="1.3"
          stroke-linejoin="round"
          fill={isBookmarked ? 'var(--accent)' : 'none'}
        />
      </svg>
    </button>

    <button
      class="icon-btn"
      on:click={() => toggleSidebar('bookmarks')}
      title="Bookmarks"
      type="button"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M3 4h10M3 8h7M3 12h5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
      </svg>
    </button>

    <button
      class="icon-btn"
      on:click={() => toggleSidebar('history')}
      title="History (Ctrl+H)"
      type="button"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.3"/>
        <path d="M8 5v3.5l2.5 1.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .navbar {
    height: var(--nav-height);
    background: var(--bg-surface);
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 10px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .nav-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }

  .icon-btn {
    width: 32px;
    height: 32px;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    transition: background 0.12s, color 0.12s;
    flex-shrink: 0;
  }

  .icon-btn:hover:not(:disabled) {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .icon-btn:disabled { opacity: 0.35; cursor: default; }
  .icon-btn.bookmarked { color: var(--accent); }

  .address-wrap {
    flex: 1;
    height: 34px;
    background: var(--bg-elevated);
    border-radius: var(--radius-lg);
    border: 1.5px solid transparent;
    display: flex;
    align-items: center;
    padding: 0 10px;
    gap: 6px;
    transition: border-color 0.15s, background 0.15s;
    overflow: hidden;
  }

  .address-wrap.focused {
    background: var(--bg-base);
    border-color: var(--border-focus);
  }

  .security-icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .security-icon.secure { color: #4caf50; }
  .security-icon.insecure { color: #f59e0b; }

  .address-input {
    flex: 1;
    font-size: 13px;
    color: var(--text-primary);
    min-width: 0;
  }

  .address-input::placeholder { color: var(--text-muted); }

  .right-actions {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }
</style>
