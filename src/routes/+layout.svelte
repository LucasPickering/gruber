<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { derived } from 'svelte/store';

  // Determine which tab is active based on the current path
  const activeTab = derived(page, ($page) => {
    if ($page.url.pathname.startsWith('/transit')) return 'transit';
    // Default to weather for root or /weather
    return 'weather';
  });

  function navigate(path: string) {
    goto(path);
  }
</script>

<nav class="tab-bar">
  <button
    class="tab"
    class:active={$activeTab === 'weather'}
    aria-current={$activeTab === 'weather' ? 'page' : undefined}
    on:click={() => navigate('/weather')}
  >
    Weather
  </button>
  <button
    class="tab"
    class:active={$activeTab === 'transit'}
    aria-current={$activeTab === 'transit' ? 'page' : undefined}
    on:click={() => navigate('/transit')}
  >
    Transit
  </button>
</nav>

<main>
  <slot />
</main>

<!-- Redirect root to /weather -->
<svelte:head>
  <script>
    if (window.location.pathname === '/') {
      window.location.replace('/weather');
    }
  </script>
</svelte:head>

<style>
  .tab-bar {
    display: flex;
    border-bottom: 2px solid #eee;
    background: #fafafa;
    padding: 0 1rem;
    margin-bottom: 2rem;
  }
  .tab {
    padding: 1rem 2rem;
    cursor: pointer;
    border: none;
    background: none;
    font-size: 1.1rem;
    font-weight: 500;
    color: #555;
    border-bottom: 2px solid transparent;
    transition:
      border-color 0.2s,
      color 0.2s;
  }
  .tab.active {
    color: #0070f3;
    border-bottom: 2px solid #0070f3;
  }
  main {
    max-width: 700px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
</style>
