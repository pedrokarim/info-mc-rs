<script lang="ts">
  import type { Snippet } from 'svelte';

  type Position = 'top' | 'bottom' | 'left' | 'right';

  let {
    text,
    position = 'top',
    delay = 200,
    children,
  }: {
    text: string;
    position?: Position;
    delay?: number;
    children: Snippet;
  } = $props();

  let visible = $state(false);
  let timer: ReturnType<typeof setTimeout> | null = null;

  function show() {
    timer = setTimeout(() => { visible = true; }, delay);
  }

  function hide() {
    if (timer) { clearTimeout(timer); timer = null; }
    visible = false;
  }
</script>

<span
  class="tooltip-wrap"
  onmouseenter={show}
  onmouseleave={hide}
  onfocusin={show}
  onfocusout={hide}
>
  {@render children()}
  {#if visible && text}
    <span class="tooltip tooltip--{position}" role="tooltip">{text}</span>
  {/if}
</span>

<style>
  .tooltip-wrap {
    position: relative;
    display: inline-flex;
  }

  .tooltip {
    position: absolute;
    z-index: 100;
    background: #161b22;
    color: #e6edf3;
    border: 1px solid #30363d;
    border-radius: 6px;
    padding: 0.3em 0.55em;
    font-size: 0.72rem;
    font-weight: 500;
    white-space: nowrap;
    pointer-events: none;
    line-height: 1.3;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: tooltip-in 120ms ease;
  }

  .tooltip--top {
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
  }
  .tooltip--bottom {
    top: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
  }
  .tooltip--left {
    right: calc(100% + 6px);
    top: 50%;
    transform: translateY(-50%);
  }
  .tooltip--right {
    left: calc(100% + 6px);
    top: 50%;
    transform: translateY(-50%);
  }

  /* Arrow */
  .tooltip::after {
    content: '';
    position: absolute;
    width: 6px;
    height: 6px;
    background: #161b22;
    border: 1px solid #30363d;
    transform: rotate(45deg);
  }
  .tooltip--top::after {
    bottom: -4px;
    left: 50%;
    margin-left: -3px;
    border-top: none;
    border-left: none;
  }
  .tooltip--bottom::after {
    top: -4px;
    left: 50%;
    margin-left: -3px;
    border-bottom: none;
    border-right: none;
  }
  .tooltip--left::after {
    right: -4px;
    top: 50%;
    margin-top: -3px;
    border-bottom: none;
    border-left: none;
  }
  .tooltip--right::after {
    left: -4px;
    top: 50%;
    margin-top: -3px;
    border-top: none;
    border-right: none;
  }

  @keyframes tooltip-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
