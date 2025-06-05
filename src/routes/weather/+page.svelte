<script lang="ts" module>
  import { invoke } from '@tauri-apps/api/core';

  console.log('invoking');
  invoke('fetch_weather').then(console.log).catch(console.error);
</script>

<h1>Weather</h1>
<p>This is the Weather page. Weather information will appear here.</p>
