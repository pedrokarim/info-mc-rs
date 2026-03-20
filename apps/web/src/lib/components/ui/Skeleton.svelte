<script lang="ts">
  let {
    width = '100%',
    height = '1rem',
    rounded = false,
    circle = false,
  }: {
    width?: string;
    height?: string;
    rounded?: boolean;
    circle?: boolean;
  } = $props();

  const radius = circle ? '50%' : rounded ? '999px' : 'var(--radius-sm, 8px)';
</script>

<div
  class="skeleton"
  style="width: {width}; height: {height}; border-radius: {radius};"
  aria-hidden="true"
></div>

<style>
  .skeleton {
    background: rgba(84, 126, 181, 0.15);
    position: relative;
    overflow: hidden;
  }

  .skeleton::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.35) 50%,
      transparent 100%
    );
    animation: shimmer 1.5s ease-in-out infinite;
  }

  @keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
</style>
