<script lang="ts">
  let {
    label = '',
    direction = 'horizontal',
    spacing = 'md',
  }: {
    label?: string;
    direction?: 'horizontal' | 'vertical';
    spacing?: 'sm' | 'md' | 'lg';
  } = $props();

  const spacings = { sm: '0.4rem', md: '0.8rem', lg: '1.5rem' };
</script>

{#if direction === 'vertical'}
  <div class="divider-v" style="margin: 0 {spacings[spacing]};"></div>
{:else if label}
  <div class="divider-labeled" style="margin: {spacings[spacing]} 0;">
    <span class="divider-line"></span>
    <span class="divider-text">{label}</span>
    <span class="divider-line"></span>
  </div>
{:else}
  <hr class="divider-h" style="margin: {spacings[spacing]} 0;" />
{/if}

<style>
  .divider-h {
    border: none;
    height: 1px;
    background: var(--line-0, rgba(46, 94, 143, 0.34));
  }

  .divider-v {
    width: 1px;
    align-self: stretch;
    background: var(--line-0, rgba(46, 94, 143, 0.34));
  }

  .divider-labeled {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }

  .divider-line {
    flex: 1;
    height: 1px;
    background: var(--line-0, rgba(46, 94, 143, 0.34));
  }

  .divider-text {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--ink-2, #5a7894);
    white-space: nowrap;
  }
</style>
