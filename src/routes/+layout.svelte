<script lang="ts">
  import { page } from '$app/stores';
  import { derived } from 'svelte/store';

  // Determine which tab is active based on the current path
  const activeTab = derived(page, ($page) => {
    if ($page.url.pathname.startsWith('/transit')) return 'transit';
    // Default to weather for root or /weather
    return 'weather';
  });
</script>

<nav class="tab-bar">
  <a
    href="/weather"
    class="tab"
    class:active={$activeTab === 'weather'}
    aria-current={$activeTab === 'weather' ? 'page' : undefined}
  >
    Weather
  </a>
  <a
    href="/transit"
    class="tab"
    class:active={$activeTab === 'transit'}
    aria-current={$activeTab === 'transit' ? 'page' : undefined}
  >
    Transit
  </a>
</nav>

<main>
  <slot />
</main>

<!-- Redirect root to /weather -->
<svelte:head>
  <script>
    if (window.location.pathname === '/') {
      window.location.pathname = '/weather';
    }
  </script>
</svelte:head>

<style>
  .tab-bar {
    display: flex;
    border-bottom: 2px solid #eee;
    background: #666;
    border: none;
    margin-bottom: 2rem;
  }

  .tab {
    flex: 1;
    padding: 1rem 2rem;
    border: none;
    background-color: none;
    font-size: 1.5rem;
    font-weight: 500;
    text-align: center;
    text-decoration: none;
    transition:
      border-color 0.2s,
      color 0.2s;
  }

  .tab.active {
    background-color: #555;
    color: #09bccb;
    border-bottom: 4px solid #09bccb;
  }

  main {
    max-width: 700px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
</style>
