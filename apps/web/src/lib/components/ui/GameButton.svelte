<script lang="ts">
  export let label = 'Action';
  export let variant: 'primary' | 'secondary' = 'primary';
  export let href: string | undefined;
  export let disabled = false;
  export let compact = false;

  $: classes = `btn btn-${variant}${compact ? ' btn-compact' : ''}`;
</script>

{#if href}
  <a class={classes} href={href} aria-disabled={disabled}>{label}</a>
{:else}
  <button class={classes} type="button" {disabled}>{label}</button>
{/if}
