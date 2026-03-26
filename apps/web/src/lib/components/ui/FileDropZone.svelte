<script lang="ts">
  let {
    accept = 'image/png',
    label = 'Glisser un fichier ici ou cliquer pour parcourir',
    multiple = false,
    onfile,
  }: {
    accept?: string;
    label?: string;
    multiple?: boolean;
    onfile?: (file: File) => void;
  } = $props();

  let dragOver = $state(false);
  let fileInput: HTMLInputElement;

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;
    const files = e.dataTransfer?.files;
    if (files && files.length > 0) {
      for (let i = 0; i < (multiple ? files.length : 1); i++) {
        onfile?.(files[i]);
      }
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  function handleClick() {
    fileInput?.click();
  }

  function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = input.files;
    if (files && files.length > 0) {
      for (let i = 0; i < (multiple ? files.length : 1); i++) {
        onfile?.(files[i]);
      }
    }
    input.value = '';
  }
</script>

<button
  class="dropzone"
  class:drag-over={dragOver}
  type="button"
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  onclick={handleClick}
  aria-label={label}
>
  <svg class="dropzone-icon" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
    <polyline points="17 8 12 3 7 8" />
    <line x1="12" y1="3" x2="12" y2="15" />
  </svg>
  <span class="dropzone-label">{label}</span>
  <input
    bind:this={fileInput}
    type="file"
    {accept}
    {multiple}
    onchange={handleFileChange}
    class="dropzone-input"
    tabindex={-1}
  />
</button>

<style>
  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    min-height: 100px;
    padding: 20px;
    border: 2px dashed var(--line-0, rgba(46, 94, 143, 0.34));
    border-radius: var(--radius-md, 12px);
    background: var(--surface-1, #edf5fa);
    color: var(--ink-2, #5a7894);
    cursor: pointer;
    transition:
      border-color var(--ease, 160ms ease),
      background var(--ease, 160ms ease),
      color var(--ease, 160ms ease);
    font-family: inherit;
    font-size: 0.82rem;
    font-weight: 500;
  }

  .dropzone:hover,
  .dropzone.drag-over {
    border-color: var(--blue-0, #5e90ff);
    background: rgba(94, 144, 255, 0.06);
    color: var(--blue-0, #5e90ff);
  }

  .dropzone:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 2px;
  }

  .dropzone-icon {
    opacity: 0.6;
  }

  .dropzone-label {
    text-align: center;
    line-height: 1.4;
  }

  .dropzone-input {
    position: absolute;
    width: 0;
    height: 0;
    opacity: 0;
    pointer-events: none;
  }
</style>
