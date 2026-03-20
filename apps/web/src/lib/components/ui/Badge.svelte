<script lang="ts">
  type Variant = 'default' | 'success' | 'warning' | 'danger' | 'info';
  type Size = 'sm' | 'md';

  let { label, variant = 'default', size = 'md' }: { label: string; variant?: Variant; size?: Size } = $props();
</script>

<span class="badge badge--{variant} badge--{size}">{label}</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    border-radius: 999px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    white-space: nowrap;
    line-height: 1;
  }

  .badge--md { font-size: 0.7rem; padding: 0.22em 0.6em; }
  .badge--sm { font-size: 0.6rem; padding: 0.18em 0.5em; }

  .badge--default {
    background: rgba(84, 126, 181, 0.15);
    color: var(--ink-1, #2d4a65);
    border: 1px solid rgba(84, 126, 181, 0.25);
  }
  .badge--success {
    background: rgba(22, 154, 96, 0.12);
    color: var(--ok, #169a60);
    border: 1px solid rgba(22, 154, 96, 0.25);
  }
  .badge--warning {
    background: rgba(210, 153, 34, 0.12);
    color: #b8860b;
    border: 1px solid rgba(210, 153, 34, 0.25);
  }
  .badge--danger {
    background: rgba(184, 59, 59, 0.12);
    color: var(--danger, #b83b3b);
    border: 1px solid rgba(184, 59, 59, 0.25);
  }
  .badge--info {
    background: rgba(94, 144, 255, 0.12);
    color: var(--blue-0, #5e90ff);
    border: 1px solid rgba(94, 144, 255, 0.25);
  }
</style>
