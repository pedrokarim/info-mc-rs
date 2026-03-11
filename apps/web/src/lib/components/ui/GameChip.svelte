<script lang="ts">
  export let label = 'Tag';
  export let href: string | undefined = undefined;
  export let active = false;
  export let disabled = false;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let onClick: ((event: MouseEvent) => void) | undefined = undefined;
  export let target: '_self' | '_blank' | '_parent' | '_top' | undefined = undefined;
  export let rel: string | undefined = undefined;
</script>

{#if href}
  <a
    class="chip"
    href={href}
    data-active={active}
    aria-current={active ? 'page' : undefined}
    {target}
    {rel}
  >
    {label}
  </a>
{:else}
  <button class="chip" {type} aria-pressed={active} {disabled} onclick={onClick}>
    {label}
  </button>
{/if}
