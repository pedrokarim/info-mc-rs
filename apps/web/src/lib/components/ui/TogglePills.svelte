<script lang="ts">
  export let options: Array<{ label: string; value: string }> = [];
  export let value = '';
  export let name = 'toggle';
  export let ariaLabel = 'Options';
</script>

<div class="mode-switch" role="radiogroup" aria-label={ariaLabel}>
  {#each options as item}
    <label class="mode-item">
      <input type="radio" bind:group={value} value={item.value} name={name} />
      <span>{item.label}</span>
    </label>
  {/each}
</div>
