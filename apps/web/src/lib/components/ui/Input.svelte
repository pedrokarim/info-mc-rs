<script lang="ts">
  type Variant = 'default' | 'error' | 'success';

  let {
    value = $bindable(''),
    label = '',
    placeholder = '',
    type = 'text',
    disabled = false,
    variant = 'default',
    hint = '',
    oninput,
  }: {
    value?: string;
    label?: string;
    placeholder?: string;
    type?: string;
    disabled?: boolean;
    variant?: Variant;
    hint?: string;
    oninput?: (value: string) => void;
  } = $props();
</script>

<div class="input-wrap">
  {#if label}
    <label class="input-label">{label}</label>
  {/if}
  <input
    class="input input--{variant}"
    {type}
    {placeholder}
    {disabled}
    bind:value
    oninput={() => oninput?.(value)}
  />
  {#if hint}
    <span class="input-hint input-hint--{variant}">{hint}</span>
  {/if}
</div>

<style>
  .input-wrap { display: flex; flex-direction: column; gap: 0.25rem; }

  .input-label {
    font-size: 0.73rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-1, #2d4a65);
  }

  .input {
    width: 100%;
    box-sizing: border-box;
    padding: 0.55rem 0.7rem;
    border: 1px solid rgba(70, 113, 166, 0.42);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-0, #0f253a);
    font-size: 0.85rem;
    font-family: inherit;
    transition: border-color 120ms ease;
  }

  .input::placeholder { color: #67839c; }
  .input:focus {
    outline: 2px solid rgba(95, 143, 255, 0.6);
    outline-offset: 1px;
    border-color: var(--blue-0, #5e90ff);
  }
  .input:disabled { opacity: 0.5; cursor: not-allowed; }

  .input--error { border-color: var(--danger, #b83b3b); }
  .input--error:focus { outline-color: rgba(184, 59, 59, 0.5); }
  .input--success { border-color: var(--ok, #169a60); }
  .input--success:focus { outline-color: rgba(22, 154, 96, 0.5); }

  .input-hint {
    font-size: 0.72rem;
    color: var(--ink-2, #5a7894);
  }
  .input-hint--error { color: var(--danger, #b83b3b); }
  .input-hint--success { color: var(--ok, #169a60); }
</style>
