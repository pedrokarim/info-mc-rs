<script lang="ts">
  import '../app.css';
  import AppHeader from '$lib/components/AppHeader.svelte';

  let { children } = $props();
</script>

<div class="app-shell">
  <AppHeader />
  {@render children()}
  <footer class="site-footer">
    <div class="site-footer-top">
      <div class="footer-col">
        <p class="footer-heading">MCInfo</p>
        <p class="footer-tagline">Statut serveur, skin viewer 3D et API ouverte pour Minecraft.</p>
      </div>
      <div class="footer-col">
        <p class="footer-heading">Navigation</p>
        <nav class="footer-links" aria-label="Footer">
          <a href="/">Accueil</a>
          <a href="/servers">Serveurs</a>
          <a href="/skins">Skins</a>
          <a href="/tools">Outils</a>
          <a href="/docs">API</a>
        </nav>
      </div>
      <div class="footer-col">
        <p class="footer-heading">Outils populaires</p>
        <nav class="footer-links" aria-label="Outils">
          <a href="/tools/skin-editor">Éditeur de Skin</a>
          <a href="/tools/motd-editor">Éditeur MOTD</a>
          <a href="/tools/banner-designer">Designer Bannière</a>
          <a href="/tools/seed-map">Carte de Seed</a>
        </nav>
      </div>
    </div>
    <div class="site-footer-bottom">
      <p>&copy; {new Date().getFullYear()} <a href="https://ascencia.re/" target="_blank" rel="noreferrer">Ascencia</a> &mdash; Développé par PedroKarim</p>
      <nav class="footer-bottom-links" aria-label="Légal">
        <a href="/contact">Contact</a>
        <a href="/mentions-legales">Mentions légales</a>
        <a href="/confidentialite">Confidentialité</a>
      </nav>
    </div>
  </footer>
</div>
