<script lang="ts">
  let {
    checked = $bindable(false),
    label = '',
    disabled = false,
    onchange,
  }: {
    checked?: boolean;
    label?: string;
    disabled?: boolean;
    onchange?: (checked: boolean) => void;
  } = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<label class="checkbox" class:disabled>
  <input type="checkbox" bind:checked class="checkbox-input" {disabled} onchange={() => onchange?.(checked)} />
  <span class="checkbox-box" class:checked>
    {#if checked}
      <svg class="checkbox-icon" viewBox="0 0 12 12" fill="none">
        <path d="M2.5 6l2.5 2.5 4.5-5" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    {/if}
  </span>
  {#if label}
    <span class="checkbox-label">{label}</span>
  {/if}
</label>

<style>
  .checkbox {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    cursor: pointer;
    user-select: none;
  }
  .checkbox.disabled { opacity: 0.5; cursor: not-allowed; }

  .checkbox-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkbox-box {
    width: 18px;
    height: 18px;
    border-radius: 4px;
    border: 1.5px solid rgba(84, 126, 181, 0.45);
    background: rgba(255, 255, 255, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 120ms ease, border-color 120ms ease;
    flex-shrink: 0;
  }

  .checkbox-box.checked {
    background: var(--blue-0, #5e90ff);
    border-color: var(--blue-1, #345fcd);
  }

  .checkbox-icon {
    width: 12px;
    height: 12px;
    color: #fff;
  }

  .checkbox-label {
    font-size: 0.85rem;
    color: var(--ink-0, #0f253a);
  }

  .checkbox-input:focus-visible + .checkbox-box {
    outline: 2px solid rgba(94, 144, 255, 0.6);
    outline-offset: 1px;
  }
</style>
