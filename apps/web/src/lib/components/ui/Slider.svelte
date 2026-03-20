<script lang="ts">
  let {
    value = $bindable(50),
    min = 0,
    max = 100,
    step = 1,
    label = '',
    showValue = true,
    disabled = false,
    onchange,
  }: {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    label?: string;
    showValue?: boolean;
    disabled?: boolean;
    onchange?: (value: number) => void;
  } = $props();

  const pct = $derived(((value - min) / (max - min)) * 100);
</script>

<div class="slider-wrap" class:disabled>
  {#if label || showValue}
    <div class="slider-header">
      {#if label}<span class="slider-label">{label}</span>{/if}
      {#if showValue}<span class="slider-value">{value}</span>{/if}
    </div>
  {/if}
  <input
    type="range"
    class="slider"
    {min}
    {max}
    {step}
    {disabled}
    bind:value
    oninput={() => onchange?.(value)}
    style="--pct: {pct}%"
  />
</div>

<style>
  .slider-wrap { display: flex; flex-direction: column; gap: 0.3rem; }
  .slider-wrap.disabled { opacity: 0.5; }

  .slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .slider-label {
    font-size: 0.73rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--ink-1, #2d4a65);
  }

  .slider-value {
    font-size: 0.78rem;
    font-family: 'JetBrains Mono', monospace;
    font-weight: 600;
    color: var(--blue-0, #5e90ff);
  }

  .slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 999px;
    background: linear-gradient(
      to right,
      var(--blue-0, #5e90ff) 0%,
      var(--blue-0, #5e90ff) var(--pct),
      rgba(84, 126, 181, 0.2) var(--pct),
      rgba(84, 126, 181, 0.2) 100%
    );
    cursor: pointer;
    outline: none;
  }

  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    border: 2px solid var(--blue-0, #5e90ff);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
    cursor: pointer;
    transition: transform 100ms ease;
  }

  .slider::-webkit-slider-thumb:hover { transform: scale(1.15); }

  .slider::-moz-range-thumb {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: #fff;
    border: 2px solid var(--blue-0, #5e90ff);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
    cursor: pointer;
  }

  .slider:focus-visible {
    outline: 2px solid rgba(94, 144, 255, 0.5);
    outline-offset: 3px;
    border-radius: 999px;
  }
</style>
