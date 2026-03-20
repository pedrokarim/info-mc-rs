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
</script>

<label class="switch" class:disabled>
  <input
    type="checkbox"
    class="switch-input"
    bind:checked
    {disabled}
    role="switch"
    aria-checked={checked}
    onchange={() => onchange?.(checked)}
  />
  <span class="switch-track" class:checked>
    <span class="switch-thumb"></span>
  </span>
  {#if label}
    <span class="switch-label">{label}</span>
  {/if}
</label>

<style>
  .switch {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    user-select: none;
  }
  .switch.disabled { opacity: 0.5; cursor: not-allowed; }

  .switch-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .switch-track {
    position: relative;
    width: 36px;
    height: 20px;
    border-radius: 999px;
    background: rgba(84, 126, 181, 0.25);
    border: 1.5px solid rgba(84, 126, 181, 0.35);
    transition: background 160ms ease, border-color 160ms ease;
    flex-shrink: 0;
  }

  .switch-track.checked {
    background: var(--blue-0, #5e90ff);
    border-color: var(--blue-1, #345fcd);
  }

  .switch-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: transform 160ms ease;
  }

  .switch-track.checked .switch-thumb {
    transform: translateX(16px);
  }

  .switch-label {
    font-size: 0.85rem;
    color: var(--ink-0, #0f253a);
  }

  .switch-input:focus-visible + .switch-track {
    outline: 2px solid rgba(94, 144, 255, 0.6);
    outline-offset: 2px;
  }
</style>
