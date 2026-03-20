<script lang="ts">
  type RadioItem = { label: string; value: string; disabled?: boolean };

  let {
    items,
    name,
    selected = $bindable(''),
    direction = 'vertical',
    onchange,
  }: {
    items: RadioItem[];
    name: string;
    selected?: string;
    direction?: 'vertical' | 'horizontal';
    onchange?: (value: string) => void;
  } = $props();

  function select(value: string) {
    selected = value;
    onchange?.(value);
  }
</script>

<div class="radio-group radio-group--{direction}" role="radiogroup">
  {#each items as item}
    <label class="radio" class:disabled={item.disabled}>
      <input
        type="radio"
        class="radio-input"
        {name}
        value={item.value}
        checked={selected === item.value}
        disabled={item.disabled}
        onchange={() => select(item.value)}
      />
      <span class="radio-circle" class:checked={selected === item.value}></span>
      <span class="radio-label">{item.label}</span>
    </label>
  {/each}
</div>

<style>
  .radio-group { display: flex; gap: 0.5rem; }
  .radio-group--vertical { flex-direction: column; }
  .radio-group--horizontal { flex-direction: row; flex-wrap: wrap; gap: 0.8rem; }

  .radio {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    cursor: pointer;
    user-select: none;
  }
  .radio.disabled { opacity: 0.5; cursor: not-allowed; }

  .radio-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .radio-circle {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 1.5px solid rgba(84, 126, 181, 0.45);
    background: rgba(255, 255, 255, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 120ms ease;
    flex-shrink: 0;
  }

  .radio-circle.checked {
    border-color: var(--blue-1, #345fcd);
  }

  .radio-circle.checked::after {
    content: '';
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--blue-0, #5e90ff);
  }

  .radio-label {
    font-size: 0.85rem;
    color: var(--ink-0, #0f253a);
  }

  .radio-input:focus-visible + .radio-circle {
    outline: 2px solid rgba(94, 144, 255, 0.6);
    outline-offset: 1px;
  }
</style>
