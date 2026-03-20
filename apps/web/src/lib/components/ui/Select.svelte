<script lang="ts">
  type Option = { label: string; value: string; disabled?: boolean };

  let {
    options,
    value = $bindable(''),
    label = '',
    placeholder = '',
    disabled = false,
    onchange,
  }: {
    options: Option[];
    value?: string;
    label?: string;
    placeholder?: string;
    disabled?: boolean;
    onchange?: (value: string) => void;
  } = $props();
</script>

<div class="select-wrap">
  {#if label}
    <label class="select-label">{label}</label>
  {/if}
  <div class="select-container">
    <select
      class="select"
      bind:value
      {disabled}
      onchange={() => onchange?.(value)}
    >
      {#if placeholder}
        <option value="" disabled>{placeholder}</option>
      {/if}
      {#each options as opt}
        <option value={opt.value} disabled={opt.disabled}>{opt.label}</option>
      {/each}
    </select>
    <span class="select-arrow">▾</span>
  </div>
</div>

<style>
  .select-wrap { display: flex; flex-direction: column; gap: 0.25rem; }

  .select-label {
    font-size: 0.73rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-1, #2d4a65);
  }

  .select-container {
    position: relative;
    display: inline-flex;
  }

  .select {
    appearance: none;
    width: 100%;
    padding: 0.55rem 2rem 0.55rem 0.7rem;
    border: 1px solid rgba(70, 113, 166, 0.42);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--ink-0, #0f253a);
    font-size: 0.85rem;
    font-family: inherit;
    cursor: pointer;
    transition: border-color 120ms ease;
  }

  .select:focus {
    outline: 2px solid rgba(95, 143, 255, 0.6);
    outline-offset: 1px;
    border-color: var(--blue-0, #5e90ff);
  }

  .select:disabled { opacity: 0.5; cursor: not-allowed; }

  .select-arrow {
    position: absolute;
    right: 0.6rem;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    color: var(--ink-2, #5a7894);
    font-size: 0.8rem;
  }
</style>
