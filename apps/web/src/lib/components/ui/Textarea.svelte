<script lang="ts">
  let {
    value = $bindable(''),
    label = '',
    placeholder = '',
    rows = 4,
    disabled = false,
    maxlength,
    oninput,
  }: {
    value?: string;
    label?: string;
    placeholder?: string;
    rows?: number;
    disabled?: boolean;
    maxlength?: number;
    oninput?: (value: string) => void;
  } = $props();
</script>

<div class="textarea-wrap">
  {#if label}
    <label class="textarea-label">{label}</label>
  {/if}
  <textarea
    class="textarea"
    {placeholder}
    {rows}
    {disabled}
    {maxlength}
    bind:value
    oninput={() => oninput?.(value)}
  ></textarea>
  {#if maxlength}
    <span class="textarea-count">{value.length}/{maxlength}</span>
  {/if}
</div>

<style>
  .textarea-wrap { display: flex; flex-direction: column; gap: 0.25rem; }

  .textarea-label {
    font-size: 0.73rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-1, #2d4a65);
  }

  .textarea {
    width: 100%;
    box-sizing: border-box;
    padding: 0.55rem 0.7rem;
    border: 1px solid rgba(70, 113, 166, 0.42);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-0, #0f253a);
    font-size: 0.85rem;
    font-family: inherit;
    resize: vertical;
    min-height: 60px;
    transition: border-color 120ms ease;
  }

  .textarea::placeholder { color: #67839c; }
  .textarea:focus {
    outline: 2px solid rgba(95, 143, 255, 0.6);
    outline-offset: 1px;
    border-color: var(--blue-0, #5e90ff);
  }
  .textarea:disabled { opacity: 0.5; cursor: not-allowed; }

  .textarea-count {
    align-self: flex-end;
    font-size: 0.68rem;
    font-family: 'JetBrains Mono', monospace;
    color: var(--ink-2, #5a7894);
  }
</style>
