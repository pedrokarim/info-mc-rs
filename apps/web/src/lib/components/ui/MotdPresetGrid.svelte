<script lang="ts">
  import type { Edition, MotdPreset } from '$lib/utils/motd';
  import { MOTD_PRESETS, toPreviewHtml, parseLegacyString } from '$lib/utils/motd';

  let {
    edition = 'java',
    onselect,
  }: {
    edition?: Edition;
    onselect?: (preset: MotdPreset) => void;
  } = $props();

  const filteredPresets = $derived(
    MOTD_PRESETS.filter(
      (p) => p.edition === 'both' || p.edition === edition,
    ),
  );

  function presetToHtml(motd: string): string {
    const lines = motd.split('\n');
    return lines
      .map((line) => toPreviewHtml(parseLegacyString(line)))
      .join('<br>');
  }
</script>

<div class="preset-grid-wrap">
  <span class="preset-label">Presets</span>

  <div class="preset-grid">
    {#each filteredPresets as preset (preset.name)}
      <button
        class="preset-card"
        onclick={() => onselect?.(preset)}
        title={preset.description}
      >
        <div class="preset-preview">
          {@html presetToHtml(preset.motd)}
        </div>
        <span class="preset-name">{preset.name}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .preset-grid-wrap {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .preset-label {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-2, #5a7894);
  }

  .preset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 10px;
  }

  .preset-card {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 0;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: 8px;
    background: none;
    cursor: pointer;
    overflow: hidden;
    transition:
      border-color 160ms ease,
      transform 160ms ease,
      box-shadow 160ms ease;
    text-align: left;
  }

  .preset-card:hover {
    border-color: var(--blue-0, #5e90ff);
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(16, 45, 72, 0.12);
  }

  .preset-card:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 2px;
  }

  .preset-preview {
    background:
      linear-gradient(rgba(0, 0, 0, 0.55), rgba(0, 0, 0, 0.55)),
      url('/images/ui/mc_dirt.jpg');
    background-size: auto, 128px 128px;
    background-repeat: repeat;
    padding: 10px 10px;
    font-family: 'Minecraft', 'JetBrains Mono', 'Courier New', monospace;
    font-size: 0.62rem;
    line-height: 1.5;
    color: #aaaaaa;
    white-space: pre-wrap;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, 0.6);
    overflow: hidden;
    text-overflow: ellipsis;
    min-height: 42px;
    text-align: center;
    image-rendering: pixelated;
  }

  .preset-preview :global(.mc-obfuscated) {
    animation: mc-scramble 100ms infinite;
    display: inline-block;
  }

  @keyframes mc-scramble {
    0% { opacity: 1; }
    50% { opacity: 0.7; }
    100% { opacity: 1; }
  }

  .preset-name {
    padding: 4px 10px 6px;
    font-family: 'Chakra Petch', sans-serif;
    font-size: 0.68rem;
    font-weight: 600;
    color: var(--ink-1, #2d4a65);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
