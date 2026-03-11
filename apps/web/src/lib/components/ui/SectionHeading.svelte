<script lang="ts">
  export let title = 'Section';
  export let description = '';
  export let light = true;
  export let level: 3 | 4 = 3;
</script>

<div class={`section-title${light ? ' section-title-light' : ''}`}>
  <div>
    {#if level === 4}
      <h4>{title}</h4>
    {:else}
      <h3>{title}</h3>
    {/if}
    {#if description}
      <p>{description}</p>
    {/if}
  </div>
  <slot name="right" />
</div>
