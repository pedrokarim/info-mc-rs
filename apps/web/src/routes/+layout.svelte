<script lang="ts">
  import '../app.css';
  import AppHeader from '$lib/components/AppHeader.svelte';

  let { children } = $props();
</script>

<div class="app-shell">
  <AppHeader />
  {@render children()}
  <footer class="site-footer">
    <div class="site-footer-inner">
      <p class="site-footer-main">MCInfo-RS</p>
      <p class="site-footer-sub">Data serveur, MOTD visuel et explorer skin Minecraft.</p>
    </div>
  </footer>
</div>
