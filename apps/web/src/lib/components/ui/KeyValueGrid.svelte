<script lang="ts">
  export let items: Array<{ key: string; value: unknown }> = [];

  function formatValue(value: unknown): string {
    if (value === null || value === undefined || value === '') return 'N/A';
    return String(value);
  }
</script>

<dl class="kv">
  {#each items as item}
    <dt>{item.key}</dt>
    <dd>{formatValue(item.value)}</dd>
  {/each}
</dl>
