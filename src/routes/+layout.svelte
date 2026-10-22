<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { theme } from '$lib/stores';

  onMount(() => {
    const unsubscribe = theme.subscribe(t => {
      if (t === 'system') document.documentElement.removeAttribute('data-theme');
      else document.documentElement.setAttribute('data-theme', t);
    });
    return unsubscribe;
  });
</script>

<slot />
