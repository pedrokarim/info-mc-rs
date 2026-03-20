<script lang="ts">
  type Size = 'sm' | 'md' | 'lg';

  let { src = '', alt = '', size = 'md', fallback = '' }: { src?: string; alt?: string; size?: Size; fallback?: string } = $props();

  let imgError = $state(false);

  const sizes: Record<Size, number> = { sm: 28, md: 40, lg: 56 };
  const fontSizes: Record<Size, string> = { sm: '0.65rem', md: '0.85rem', lg: '1.15rem' };

  function initials(text: string): string {
    return text.slice(0, 2).toUpperCase();
  }
</script>

{#if src && !imgError}
  <img
    class="avatar avatar--{size}"
    {src}
    alt={alt || fallback}
    width={sizes[size]}
    height={sizes[size]}
    onerror={() => { imgError = true; }}
  />
{:else}
  <div
    class="avatar avatar--{size} avatar--fallback"
    style="font-size: {fontSizes[size]}"
    role="img"
    aria-label={alt || fallback}
  >
    {initials(fallback || alt || '?')}
  </div>
{/if}

<style>
  .avatar {
    border-radius: 50%;
    border: 2px solid var(--line-0, rgba(46, 94, 143, 0.34));
    object-fit: cover;
    flex-shrink: 0;
  }

  .avatar--sm { width: 28px; height: 28px; }
  .avatar--md { width: 40px; height: 40px; }
  .avatar--lg { width: 56px; height: 56px; }

  .avatar--fallback {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--surface-2, #d8ebf6);
    color: var(--ink-1, #2d4a65);
    font-family: 'Teko', sans-serif;
    font-weight: 600;
    letter-spacing: 0.02em;
    line-height: 1;
  }
</style>
