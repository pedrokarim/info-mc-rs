<script lang="ts">
  import { editorState } from '$lib/stores/skin-editor.svelte';
  import { BODY_PARTS } from '$lib/utils/skin-uv-regions';

  function selectPart(id: string) {
    editorState.selectedBodyPart = editorState.selectedBodyPart === id ? null : id;

    if (editorState.selectedBodyPart) {
      const part = BODY_PARTS.find((p) => p.id === editorState.selectedBodyPart);
      if (part) {
        const region = editorState.activeLayer === 'base' ? part.base : part.overlay;
        editorState.panOffset.x = -(region.x * editorState.zoom) + 100;
        editorState.panOffset.y = -(region.y * editorState.zoom) + 60;
      }
    }
  }

  const parts = [
    { id: 'head', x: 16, y: 0, w: 32, h: 32, label: 'Tête' },
    { id: 'body', x: 16, y: 32, w: 32, h: 48, label: 'Torse' },
    { id: 'rarm', x: 0, y: 32, w: 16, h: 48, label: 'Bras D' },
    { id: 'larm', x: 48, y: 32, w: 16, h: 48, label: 'Bras G' },
    { id: 'rleg', x: 16, y: 80, w: 16, h: 48, label: 'Jambe D' },
    { id: 'lleg', x: 32, y: 80, w: 16, h: 48, label: 'Jambe G' },
  ];
</script>

<div class="body-map">
  <span class="map-title">Parties du corps</span>
  <svg viewBox="0 0 64 128" class="map-svg" role="img" aria-label="Carte du corps Minecraft">
    {#each parts as part}
      {@const bpDef = BODY_PARTS.find((p) => p.id === part.id)}
      <rect
        x={part.x}
        y={part.y}
        width={part.w}
        height={part.h}
        fill={bpDef?.color ?? '#888'}
        fill-opacity={editorState.selectedBodyPart === part.id ? 0.7 : 0.35}
        stroke={bpDef?.color ?? '#888'}
        stroke-width={editorState.selectedBodyPart === part.id ? 2 : 0.8}
        rx="1"
        class="map-part"
        role="button"
        tabindex={0}
        onclick={() => selectPart(part.id)}
        onkeydown={(e) => e.key === 'Enter' && selectPart(part.id)}
      />
      <text
        x={part.x + part.w / 2}
        y={part.y + part.h / 2 + 3}
        text-anchor="middle"
        class="map-label"
        pointer-events="none"
      >{part.label}</text>
    {/each}
  </svg>
</div>

<style>
  .body-map { display: flex; flex-direction: column; gap: 6px; align-items: center; }
  .map-title { font-size: 0.7rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.06em; color: var(--ink-2, #5a7894); }
  .map-svg { width: 100%; max-width: 120px; }
  .map-part { cursor: pointer; transition: fill-opacity 160ms ease; }
  .map-part:hover { fill-opacity: 0.55; }
  .map-label { font-size: 6px; font-weight: 600; font-family: 'Chakra Petch', sans-serif; fill: #fff; }
</style>
