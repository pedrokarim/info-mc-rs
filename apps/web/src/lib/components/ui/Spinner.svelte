<script lang="ts">
  type Size = 'sm' | 'md' | 'lg';

  let {
    size = 'md',
    label = 'Chargement...',
  }: {
    size?: Size;
    label?: string;
  } = $props();

  const sizes: Record<Size, number> = { sm: 16, md: 24, lg: 40 };
</script>

<span class="spinner" role="status" aria-label={label}>
  <svg
    width={sizes[size]}
    height={sizes[size]}
    viewBox="0 0 24 24"
    fill="none"
    class="spinner-svg"
  >
    <circle cx="12" cy="12" r="10" stroke="rgba(84,126,181,0.2)" stroke-width="3" />
    <path d="M12 2a10 10 0 019.8 8" stroke="var(--blue-0, #5e90ff)" stroke-width="3" stroke-linecap="round" />
  </svg>
</span>

<style>
  .spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .spinner-svg {
    animation: spin 800ms linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
