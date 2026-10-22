<script lang="ts">
  import { bookmarks, history } from '../stores';
  import { navigate } from '../browser';
  import { activeTab } from '../stores';
  import { onDestroy } from 'svelte';

  const quickLinks = [
    { title: 'YouTube', url: 'https://youtube.com', icon: '▶' },
    { title: 'GitHub', url: 'https://github.com', icon: '🐙' },
    { title: 'Gmail', url: 'https://mail.google.com', icon: '✉' },
    { title: 'Maps', url: 'https://maps.google.com', icon: '📍' },
    { title: 'Twitter/X', url: 'https://x.com', icon: '𝕏' },
    { title: 'Reddit', url: 'https://reddit.com', icon: '🔴' },
  ];

  let searchQuery = '';

  function handleSearch() {
    if (!searchQuery.trim() || !$activeTab) return;
    navigate($activeTab.id, searchQuery);
  }

  $: recentHistory = $history.slice(0, 6);
  $: recentBookmarks = $bookmarks.slice(0, 6);

  function getTime() { return new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }); }
  let time = getTime();
  const interval = setInterval(() => { time = getTime(); }, 10000);
  onDestroy(() => clearInterval(interval));
</script>

<div class="newtab">
  <div class="hero">
    <div class="clock">{time}</div>
    <div class="date">{new Date().toLocaleDateString(undefined, { weekday: 'long', month: 'long', day: 'numeric' })}</div>
    <div class="search-wrap">
      <svg class="search-icon" width="18" height="18" viewBox="0 0 18 18" fill="none">
        <circle cx="8" cy="8" r="5.5" stroke="currentColor" stroke-width="1.5"/>
        <path d="M12 12l3.5 3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
      <input bind:value={searchQuery} class="search-input" type="text" placeholder="Search or enter address"
        on:keydown={(e) => e.key === 'Enter' && handleSearch()} autocomplete="off" spellcheck="false" />
    </div>
  </div>

  <div class="sections">
    <section>
      <h2 class="section-title">Quick Access</h2>
      <div class="grid">
        {#each quickLinks as link}
          <button class="card" on:click={() => $activeTab && navigate($activeTab.id, link.url)} type="button">
            <span class="card-icon">{link.icon}</span>
            <span class="card-title">{link.title}</span>
          </button>
        {/each}
      </div>
    </section>
    {#if recentBookmarks.length > 0}
      <section>
        <h2 class="section-title">Bookmarks</h2>
        <div class="list">
          {#each recentBookmarks as bm}
            <button class="list-item" on:click={() => $activeTab && navigate($activeTab.id, bm.url)} type="button">
              {#if bm.favicon}<img src={bm.favicon} alt="" width="16" height="16" />{:else}<span class="list-icon">⭐</span>{/if}
              <span class="list-title">{bm.title || bm.url}</span>
              <span class="list-url">{bm.url}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}
    {#if recentHistory.length > 0}
      <section>
        <h2 class="section-title">Recent</h2>
        <div class="list">
          {#each recentHistory as entry}
            <button class="list-item" on:click={() => $activeTab && navigate($activeTab.id, entry.url)} type="button">
              {#if entry.favicon}<img src={entry.favicon} alt="" width="16" height="16" />{:else}<span class="list-icon">🕐</span>{/if}
              <span class="list-title">{entry.title || entry.url}</span>
              <span class="list-url">{entry.url}</span>
            </button>
          {/each}
        </div>
      </section>
    {/if}
  </div>
</div>

<style>
  .newtab { flex: 1; overflow-y: auto; background: var(--bg-base); display: flex; flex-direction: column; align-items: center; padding: 60px 24px 40px; gap: 40px; }
  .hero { display: flex; flex-direction: column; align-items: center; gap: 12px; width: 100%; max-width: 580px; }
  .clock { font-size: 72px; font-weight: 200; letter-spacing: -2px; color: var(--text-primary); line-height: 1; }
  .date { color: var(--text-secondary); font-size: 15px; margin-bottom: 12px; }
  .search-wrap { width: 100%; height: 46px; background: var(--bg-elevated); border-radius: var(--radius-full); border: 1.5px solid var(--border); display: flex; align-items: center; padding: 0 16px; gap: 10px; transition: border-color 0.15s; }
  .search-wrap:focus-within { border-color: var(--border-focus); background: var(--bg-surface); }
  .search-icon { color: var(--text-muted); flex-shrink: 0; }
  .search-input { flex: 1; font-size: 15px; color: var(--text-primary); user-select: text; }
  .search-input::placeholder { color: var(--text-muted); }
  .sections { width: 100%; max-width: 680px; display: flex; flex-direction: column; gap: 32px; }
  .section-title { font-size: 11px; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; color: var(--text-muted); margin-bottom: 10px; }
  .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(96px, 1fr)); gap: 8px; }
  .card { display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px 8px; background: var(--bg-surface); border: 1px solid var(--border); border-radius: var(--radius); transition: background 0.12s, transform 0.1s; cursor: pointer; }
  .card:hover { background: var(--bg-elevated); transform: translateY(-1px); }
  .card-icon { font-size: 22px; }
  .card-title { font-size: 11px; color: var(--text-secondary); }
  .list { display: flex; flex-direction: column; gap: 2px; }
  .list-item { display: flex; align-items: center; gap: 10px; padding: 8px 10px; border-radius: var(--radius-sm); transition: background 0.1s; text-align: left; }
  .list-item:hover { background: var(--bg-elevated); }
  .list-icon { font-size: 14px; flex-shrink: 0; width: 16px; text-align: center; }
  .list-title { font-size: 13px; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; flex: 1; }
  .list-url { font-size: 11px; color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 180px; flex-shrink: 0; }
</style>
