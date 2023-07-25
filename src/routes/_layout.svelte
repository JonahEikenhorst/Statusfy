<script lang="ts">
export const prerender = true
export const ssr = false

// With the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'

document.addEventListener('DOMContentLoaded', async () => {
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  await invoke('close_splashscreen')
});
</script>