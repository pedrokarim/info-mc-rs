<script lang="ts">
  interface Props {
    html?: string;
    fallback?: string;
  }

  let { html = '', fallback = 'No MOTD' }: Props = $props();
</script>

<div class="motd-frame">
  <div class="motd-inner">
    {#if html}
      {@html html}
    {:else}
      <span class="motd-fallback">{fallback}</span>
    {/if}
  </div>
</div>

<style>
  .motd-frame {
    background: #2b2b2b;
    border: 2px solid #1a1a1a;
    border-radius: 6px;
    padding: 2px;
    box-shadow:
      inset 0 0 0 1px rgba(255, 255, 255, 0.06),
      0 2px 8px rgba(0, 0, 0, 0.3);
    overflow: hidden;
  }

  .motd-inner {
    background:
      linear-gradient(135deg, rgba(60, 60, 60, 0.4) 0%, transparent 50%),
      #313131;
    padding: 8px 12px;
    font-family: 'JetBrains Mono', 'Courier New', monospace;
    font-size: 0.78rem;
    line-height: 1.4;
    color: #aaaaaa;
    white-space: pre-wrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, 0.4);
    text-align: center;
  }

  /* Minecraft obfuscated text animation */
  .motd-inner :global(.mc-obfuscated) {
    animation: mc-scramble 100ms infinite;
    display: inline-block;
  }

  @keyframes mc-scramble {
    0% { opacity: 1; }
    50% { opacity: 0.7; }
    100% { opacity: 1; }
  }

  .motd-fallback {
    color: #555555;
    font-style: italic;
  }
</style>
